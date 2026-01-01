use serde::Deserialize;

/// Error response from Firebase REST API
#[derive(Deserialize)]
pub struct FirebaseAuthErrorResponse {
    pub error: FirebaseAuthErrorDetail,
}

#[derive(Deserialize)]
pub struct FirebaseAuthErrorDetail {
    pub code: u32,
    pub message: String,
}
