use serde::Deserialize;

/// Claims from a Firebase ID token
/// Reference: https://firebase.google.com/docs/auth/admin/verify-id-tokens
#[derive(Debug, Deserialize)]
pub struct FirebaseClaims {
    pub email: Option<String>,
    pub sub: String,
    pub aud: String,
    pub iss: String,
    pub exp: u64,
}
