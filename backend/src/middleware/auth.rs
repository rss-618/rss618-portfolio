use tonic::{Request, Status};

use crate::state::AppState;
use crate::utils::{parse_cookie, ID_TOKEN_COOKIE};

/// Sync auth interceptor for protected gRPC services.
///
/// Uses cached Firebase public keys for JWT verification. If keys are missing
/// or the key ID isn't found (due to key rotation), verification fails and the
/// frontend's refresh flow will fetch fresh keys. Not the most precise or
/// performant approach, but keeps things simple for a single-user admin panel.
pub fn auth_interceptor(
    state: AppState,
) -> impl Fn(Request<()>) -> Result<Request<()>, Status> + Clone {
    move |req: Request<()>| {
        let cookie_header = req
            .metadata()
            .get("cookie")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        let id_token = parse_cookie(cookie_header, ID_TOKEN_COOKIE)
            .ok_or_else(|| Status::unauthenticated("Missing authentication token"))?;

        state
            .firebase_auth
            .verify_token_sync(&id_token)
            .map_err(|_| Status::unauthenticated("Invalid or expired token"))?;

        Ok(req)
    }
}
