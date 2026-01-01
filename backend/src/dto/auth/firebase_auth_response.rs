use serde::Deserialize;

/// Response from Firebase REST API on successful login
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirebaseAuthResponse {
    pub id_token: String,
    pub refresh_token: String,
    pub expires_in: String,
    pub local_id: String,
    pub email: String,
}
