#[derive(Debug, Clone, PartialEq)]
pub enum AuthError {
    InvalidToken,
    Unauthorized,
    KeyFetchFailed,
}
