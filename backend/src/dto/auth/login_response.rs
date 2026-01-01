use serde::Serialize;

/// Response sent to frontend on successful login
#[derive(Serialize)]
pub struct LoginResponse {
    pub uid: String,
    pub email: String,
}
