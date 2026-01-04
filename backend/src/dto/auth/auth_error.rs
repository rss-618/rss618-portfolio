#[derive(Debug, Clone, PartialEq)]
pub enum AuthError {
    InvalidToken,
    ExpiredToken,
    InvalidCredentials,
    InvalidRefreshToken,
    UserDisabled,
    TooManyAttempts,
    NetworkError,
    InvalidResponse,
    KeyFetchFailed,
}

impl AuthError {
    pub fn from_firebase_message(message: &str) -> Self {
        match message {
            "EMAIL_NOT_FOUND" | "INVALID_PASSWORD" | "INVALID_LOGIN_CREDENTIALS" => {
                Self::InvalidCredentials
            }
            "USER_DISABLED" => Self::UserDisabled,
            "TOO_MANY_ATTEMPTS_TRY_LATER" => Self::TooManyAttempts,
            _ => Self::InvalidCredentials,
        }
    }
}
