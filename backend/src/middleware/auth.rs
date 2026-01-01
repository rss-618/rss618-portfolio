use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use axum_extra::extract::CookieJar;

use crate::{
    dto::auth::{AuthError, TokenRefreshResponse},
    services::FirebaseAuth,
    state::AppState,
    utils::set_auth_cookies,
};

/// Middleware that requires a valid Firebase ID token from cookies.
/// If the token is expired but refresh_token is valid, it will auto-refresh
/// and set new cookies on the response.
pub async fn require_auth(
    State(state): State<AppState>,
    jar: CookieJar,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let id_token = jar.get("id_token").map(|c| c.value().to_string());
    let refresh_token = jar.get("refresh_token").map(|c| c.value().to_string());

    let id_token = id_token.ok_or(StatusCode::UNAUTHORIZED)?;

    match state.firebase_auth.verify_token(&id_token).await {
        Ok(_claims) => Ok(next.run(request).await),
        Err(AuthError::ExpiredToken) => {
            let refresh_token = refresh_token.ok_or(StatusCode::UNAUTHORIZED)?;
            let refreshed = attempt_refresh(&state.firebase_auth, &refresh_token).await?;

            let mut response = next.run(request).await;
            let expires_in = refreshed.expires_in.parse().unwrap_or(3600);
            set_auth_cookies(
                &mut response,
                &refreshed.id_token,
                &refreshed.refresh_token,
                expires_in,
            );

            Ok(response)
        }
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}

async fn attempt_refresh(
    firebase_auth: &FirebaseAuth,
    refresh_token: &str,
) -> Result<TokenRefreshResponse, StatusCode> {
    firebase_auth
        .refresh_token(refresh_token)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)
}
