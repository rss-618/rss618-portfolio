use serde::Deserialize;

/// Request from frontend to login
#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}
