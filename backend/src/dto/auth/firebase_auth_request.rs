use serde::Serialize;

/// Request sent to Firebase REST API
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FirebaseAuthRequest {
    pub email: String,
    pub password: String,
    pub return_secure_token: bool,
}
