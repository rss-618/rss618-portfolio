pub const ID_TOKEN_COOKIE: &str = "id_token";
pub const REFRESH_TOKEN_COOKIE: &str = "refresh_token";
/// Refresh tokens don't expire, but we set a reasonable max age (30 days)
pub const REFRESH_TOKEN_MAX_AGE: u64 = 30 * 24 * 60 * 60;

/// Parse a specific cookie value from a Cookie header string
pub fn parse_cookie(cookie_header: &str, name: &str) -> Option<String> {
    cookie_header
        .split(';')
        .filter_map(|part| {
            let mut parts = part.trim().splitn(2, '=');
            let key = parts.next()?;
            let value = parts.next()?;
            if key == name {
                Some(value.to_string())
            } else {
                None
            }
        })
        .next()
}

/// Build Set-Cookie strings for auth tokens
fn build_auth_cookies(
    id_token: &str,
    refresh_token: &str,
    id_token_expires_in: u64,
) -> (String, String) {
    let id_cookie = format!(
        "{}={}; Path=/; Max-Age={}; HttpOnly; SameSite=Strict; Secure",
        ID_TOKEN_COOKIE, id_token, id_token_expires_in
    );
    let refresh_cookie = format!(
        "{}={}; Path=/; Max-Age={}; HttpOnly; SameSite=Strict; Secure",
        REFRESH_TOKEN_COOKIE, refresh_token, REFRESH_TOKEN_MAX_AGE
    );
    (id_cookie, refresh_cookie)
}

/// Build Set-Cookie strings to clear auth cookies
fn build_clear_auth_cookies() -> (String, String) {
    let id_cookie = format!(
        "{}=; Path=/; Max-Age=0; HttpOnly; SameSite=Strict; Secure",
        ID_TOKEN_COOKIE
    );
    let refresh_cookie = format!(
        "{}=; Path=/; Max-Age=0; HttpOnly; SameSite=Strict; Secure",
        REFRESH_TOKEN_COOKIE
    );
    (id_cookie, refresh_cookie)
}

/// Set auth cookies on a tonic gRPC response
pub fn set_grpc_auth_cookies<T>(
    response: &mut tonic::Response<T>,
    id_token: &str,
    refresh_token: &str,
    id_token_expires_in: u64,
) {
    let (id_cookie, refresh_cookie) =
        build_auth_cookies(id_token, refresh_token, id_token_expires_in);
    response
        .metadata_mut()
        .insert("set-cookie", id_cookie.parse().unwrap());
    response
        .metadata_mut()
        .append("set-cookie", refresh_cookie.parse().unwrap());
}

/// Clear auth cookies on a tonic gRPC response
pub fn clear_grpc_auth_cookies<T>(response: &mut tonic::Response<T>) {
    let (id_cookie, refresh_cookie) = build_clear_auth_cookies();
    response
        .metadata_mut()
        .insert("set-cookie", id_cookie.parse().unwrap());
    response
        .metadata_mut()
        .append("set-cookie", refresh_cookie.parse().unwrap());
}

/// Set auth cookies on an HTTP response
pub fn set_http_auth_cookies<B>(
    response: &mut http::Response<B>,
    id_token: &str,
    refresh_token: &str,
    id_token_expires_in: u64,
) {
    let (id_cookie, refresh_cookie) =
        build_auth_cookies(id_token, refresh_token, id_token_expires_in);
    let headers = response.headers_mut();
    headers.insert(
        http::header::SET_COOKIE,
        id_cookie.parse().unwrap(),
    );
    headers.append(
        http::header::SET_COOKIE,
        refresh_cookie.parse().unwrap(),
    );
}

/// Clear auth cookies on an HTTP response
pub fn clear_http_auth_cookies<B>(response: &mut http::Response<B>) {
    let (id_cookie, refresh_cookie) = build_clear_auth_cookies();
    let headers = response.headers_mut();
    headers.insert(http::header::SET_COOKIE, id_cookie.parse().unwrap());
    headers.append(http::header::SET_COOKIE, refresh_cookie.parse().unwrap());
}
