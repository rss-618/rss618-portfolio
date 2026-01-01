use jsonwebtoken::{decode, decode_header, DecodingKey, Validation};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::dto::{AuthError, FirebaseClaims, FirebaseKeys};

/// Google's public key endpoint for Firebase token verification
/// Reference: https://firebase.google.com/docs/auth/admin/verify-id-tokens
const FIREBASE_PUBLIC_KEYS_URL: &str =
    "https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com";

/// Firebase secure token issuer base URL
const FIREBASE_ISSUER_BASE: &str = "https://securetoken.google.com";

pub struct FirebaseAuth {
    project_id: String,
    keys: Arc<RwLock<Option<FirebaseKeys>>>,
}

impl FirebaseAuth {
    pub fn new(project_id: String) -> Self {
        Self {
            project_id,
            keys: Arc::new(RwLock::new(None)),
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

        let token_data = decode::<FirebaseClaims>(token, key, &validation)
            .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
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
