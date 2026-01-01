use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};

use crate::dto::auth::{AuthError, LoginRequest};
use crate::state::AppState;
use crate::utils::{clear_auth_cookies, set_auth_cookies};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/auth/login", post(login))
        .route("/auth/logout", post(logout))
}

async fn login(State(state): State<AppState>, Json(payload): Json<LoginRequest>) -> Response {
    match state
        .firebase_auth
        .login(payload.email, payload.password)
        .await
    {
        Ok(auth_response) => {
            let expires_in = auth_response.expires_in.parse().unwrap_or(3600);
            let mut response = StatusCode::OK.into_response();
            set_auth_cookies(
                &mut response,
                &auth_response.id_token,
                &auth_response.refresh_token,
                expires_in,
            );
            response
        }
        Err(err) => {
            let status = match err {
                AuthError::InvalidCredentials => StatusCode::UNAUTHORIZED,
                AuthError::UserDisabled => StatusCode::FORBIDDEN,
                AuthError::TooManyAttempts => StatusCode::TOO_MANY_REQUESTS,
                AuthError::NetworkError | AuthError::InvalidResponse => {
                    StatusCode::INTERNAL_SERVER_ERROR
                }
                _ => StatusCode::BAD_REQUEST,
            };
            (status, format!("{:?}", err)).into_response()
        }
    }
}

async fn logout() -> Response {
    let mut response = (StatusCode::OK, "Logged out").into_response();
    clear_auth_cookies(&mut response);
    response
}
