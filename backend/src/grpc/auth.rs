use proto::auth::auth_service_server::AuthService as AuthServiceTrait;
use proto::auth::{
    LoginRequest, LoginResponse, LogoutRequest, LogoutResponse, RefreshTokenRequest,
    RefreshTokenResponse,
};
use tonic::{Request, Response, Status};

use crate::dto::auth::AuthError;
use crate::state::AppState;
use crate::utils::{
    clear_grpc_auth_cookies, parse_cookie, set_grpc_auth_cookies, REFRESH_TOKEN_COOKIE,
};

pub struct AuthController {
    state: AppState,
}

impl AuthController {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }
}

#[tonic::async_trait]
impl AuthServiceTrait for AuthController {
    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        let req = request.into_inner();

        let auth_response = self
            .state
            .firebase_auth
            .login(req.email, req.password)
            .await
            .map_err(|e| match e {
                AuthError::InvalidCredentials => Status::unauthenticated("Invalid credentials"),
                AuthError::UserDisabled => Status::permission_denied("User disabled"),
                AuthError::TooManyAttempts => {
                    Status::resource_exhausted("Too many attempts, try later")
                }
                AuthError::NetworkError | AuthError::InvalidResponse => {
                    Status::internal("Authentication service error")
                }
                _ => Status::internal("Authentication failed"),
            })?;

        let expires_in: u64 = auth_response.expires_in.parse().unwrap_or(3600);

        let mut response = Response::new(LoginResponse {});
        set_grpc_auth_cookies(
            &mut response,
            &auth_response.id_token,
            &auth_response.refresh_token,
            expires_in,
        );

        Ok(response)
    }

    async fn logout(
        &self,
        _request: Request<LogoutRequest>,
    ) -> Result<Response<LogoutResponse>, Status> {
        let mut response = Response::new(LogoutResponse {});
        clear_grpc_auth_cookies(&mut response);

        Ok(response)
    }

    async fn refresh_token(
        &self,
        request: Request<RefreshTokenRequest>,
    ) -> Result<Response<RefreshTokenResponse>, Status> {
        let cookie_header = request
            .metadata()
            .get("cookie")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        let refresh_token = parse_cookie(cookie_header, REFRESH_TOKEN_COOKIE)
            .ok_or_else(|| Status::unauthenticated("Missing refresh token"))?;

        let refreshed = self
            .state
            .firebase_auth
            .refresh_token(&refresh_token)
            .await
            .map_err(|e| match e {
                AuthError::InvalidRefreshToken => Status::unauthenticated("Invalid refresh token"),
                AuthError::ExpiredToken => Status::unauthenticated("Refresh token expired"),
                _ => Status::internal("Token refresh failed"),
            })?;

        let expires_in: u64 = refreshed.expires_in.parse().unwrap_or(3600);

        let mut response = Response::new(RefreshTokenResponse {});
        set_grpc_auth_cookies(
            &mut response,
            &refreshed.id_token,
            &refreshed.refresh_token,
            expires_in,
        );

        Ok(response)
    }
}
