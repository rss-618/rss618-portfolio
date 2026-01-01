use jsonwebtoken::{decode, decode_header, errors::ErrorKind, DecodingKey, Validation};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::dto::auth::{
    AuthError, FirebaseAuthErrorResponse, FirebaseAuthRequest, FirebaseAuthResponse,
    FirebaseClaims, FirebaseKeys, TokenRefreshResponse,
};

/// Google's public key endpoint for Firebase token verification
/// Reference: https://firebase.google.com/docs/auth/admin/verify-id-tokens
const FIREBASE_PUBLIC_KEYS_URL: &str =
    "https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com";

/// Firebase secure token issuer base URL
/// Reference: https://firebase.google.com/docs/auth/admin/verify-id-tokens
const FIREBASE_ISSUER_BASE: &str = "https://securetoken.google.com";

/// Firebase REST API for password sign-in (v1 is required - signInWithPassword not available in v2)
/// Reference: https://firebase.google.com/docs/reference/rest/auth
const FIREBASE_AUTH_URL: &str =
    "https://identitytoolkit.googleapis.com/v1/accounts:signInWithPassword";

/// Firebase REST API for token refresh
/// Reference: https://firebase.google.com/docs/reference/rest/auth#section-refresh-token
const FIREBASE_TOKEN_URL: &str = "https://securetoken.googleapis.com/v1/token";

pub struct FirebaseAuth {
    project_id: String,
    api_key: String,
    keys: Arc<RwLock<Option<FirebaseKeys>>>,
    http_client: reqwest::Client,
}

impl FirebaseAuth {
    pub fn new(project_id: String, api_key: String) -> Self {
        Self {
            project_id,
            api_key,
            keys: Arc::new(RwLock::new(None)),
            http_client: reqwest::Client::new(),
        }
    }

    /// Authenticate user with email/password via Firebase REST API
    pub async fn login(
        &self,
        email: String,
        password: String,
    ) -> Result<FirebaseAuthResponse, AuthError> {
        let request = FirebaseAuthRequest {
            email,
            password,
            return_secure_token: true,
        };

        let url = format!("{}?key={}", FIREBASE_AUTH_URL, self.api_key);

        let response = self
            .http_client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|_| AuthError::NetworkError)?;

        if response.status().is_success() {
            response
                .json::<FirebaseAuthResponse>()
                .await
                .map_err(|_| AuthError::InvalidResponse)
        } else {
            let error_response = response
                .json::<FirebaseAuthErrorResponse>()
                .await
                .map_err(|_| AuthError::InvalidCredentials)?;

            Err(AuthError::from_firebase_message(
                &error_response.error.message,
            ))
        }
    }

    pub async fn verify_token(&self, token: &str) -> Result<FirebaseClaims, AuthError> {
        let header = decode_header(token).map_err(|_| AuthError::InvalidToken)?;
        let kid = header.kid.ok_or(AuthError::InvalidToken)?;

        let keys = self.get_keys().await?;
        let key = keys.keys.get(&kid).ok_or(AuthError::InvalidToken)?;

        let mut validation = Validation::new(jsonwebtoken::Algorithm::RS256);
        validation.set_audience(&[&self.project_id]);
        validation.set_issuer(&[format!("{}/{}", FIREBASE_ISSUER_BASE, self.project_id)]);

        let token_data = decode::<FirebaseClaims>(token, key, &validation).map_err(|e| {
            match e.kind() {
                ErrorKind::ExpiredSignature => AuthError::ExpiredToken,
                _ => AuthError::InvalidToken,
            }
        })?;

        Ok(token_data.claims)
    }

    /// Refresh an expired ID token using a refresh token
    pub async fn refresh_token(&self, refresh_token: &str) -> Result<TokenRefreshResponse, AuthError> {
        let url = format!("{}?key={}", FIREBASE_TOKEN_URL, self.api_key);

        let params = [
            ("grant_type", "refresh_token"),
            ("refresh_token", refresh_token),
        ];

        let response = self
            .http_client
            .post(&url)
            .form(&params)
            .send()
            .await
            .map_err(|_| AuthError::NetworkError)?;

        if response.status().is_success() {
            response
                .json::<TokenRefreshResponse>()
                .await
                .map_err(|_| AuthError::InvalidResponse)
        } else {
            Err(AuthError::InvalidToken)
        }
    }

    async fn get_keys(&self) -> Result<FirebaseKeys, AuthError> {
        if let Some(keys) = self.keys.read().await.clone() {
            return Ok(keys);
        }

        let keys = self.fetch_keys().await?;
        *self.keys.write().await = Some(keys.clone());
        Ok(keys)
    }

    async fn fetch_keys(&self) -> Result<FirebaseKeys, AuthError> {
        let response: HashMap<String, String> = reqwest::get(FIREBASE_PUBLIC_KEYS_URL)
            .await
            .map_err(|_| AuthError::KeyFetchFailed)?
            .json()
            .await
            .map_err(|_| AuthError::KeyFetchFailed)?;

        let mut keys = HashMap::new();
        for (kid, cert) in response {
            if let Ok(key) = DecodingKey::from_rsa_pem(cert.as_bytes()) {
                keys.insert(kid, key);
            }
        }

        Ok(FirebaseKeys { keys })
    }
}
