use axum::{
    http::{header::SET_COOKIE, HeaderValue},
    response::Response,
};

const ID_TOKEN_COOKIE: &str = "id_token";
const REFRESH_TOKEN_COOKIE: &str = "refresh_token";
/// Refresh tokens don't expire, but we set a reasonable max age (30 days)
const REFRESH_TOKEN_MAX_AGE: u64 = 30 * 24 * 60 * 60;

/// Set auth cookies (id_token and refresh_token) on a response.
/// `id_token_expires_in` comes from Firebase response (typically 3600 seconds).
pub fn set_auth_cookies(
    response: &mut Response,
    id_token: &str,
    refresh_token: &str,
    id_token_expires_in: u64,
) {
    let headers = response.headers_mut();

    let id_cookie = format!(
        "{}={}; Path=/; Max-Age={}; HttpOnly; SameSite=Strict; Secure",
        ID_TOKEN_COOKIE, id_token, id_token_expires_in
    );
    headers.append(SET_COOKIE, HeaderValue::from_str(&id_cookie).unwrap());

    let refresh_cookie = format!(
        "{}={}; Path=/; Max-Age={}; HttpOnly; SameSite=Strict; Secure",
        REFRESH_TOKEN_COOKIE, refresh_token, REFRESH_TOKEN_MAX_AGE
    );
    headers.append(SET_COOKIE, HeaderValue::from_str(&refresh_cookie).unwrap());
}

/// Clear auth cookies on a response
pub fn clear_auth_cookies(response: &mut Response) {
    let headers = response.headers_mut();

    let id_cookie = format!(
        "{}=; Path=/; Max-Age=0; HttpOnly; SameSite=Strict; Secure",
        ID_TOKEN_COOKIE
    );
    headers.append(SET_COOKIE, HeaderValue::from_str(&id_cookie).unwrap());

    let refresh_cookie = format!(
        "{}=; Path=/; Max-Age=0; HttpOnly; SameSite=Strict; Secure",
        REFRESH_TOKEN_COOKIE
    );
    headers.append(SET_COOKIE, HeaderValue::from_str(&refresh_cookie).unwrap());
}
