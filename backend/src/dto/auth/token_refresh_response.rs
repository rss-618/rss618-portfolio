use serde::Deserialize;

#[derive(Deserialize)]
pub struct TokenRefreshResponse {
    pub id_token: String,
    pub refresh_token: String,
    pub expires_in: String,
}
