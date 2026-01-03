use std::future::Future;
use std::pin::Pin;

use axum::body::Body;
use http::{Request, Response};
use tonic::Status;
use tower_http::auth::AsyncAuthorizeRequest;

use crate::state::AppState;
use crate::utils::{parse_cookie, ID_TOKEN_COOKIE};

/// Async auth middleware using tower-http's AsyncAuthorizeRequest.
#[derive(Clone)]
pub struct GrpcAuth {
    state: AppState,
}

impl GrpcAuth {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }
}

impl<B: Send + 'static> AsyncAuthorizeRequest<B> for GrpcAuth {
    type RequestBody = B;
    type ResponseBody = Body;
    type Future = Pin<Box<dyn Future<Output = Result<Request<B>, Response<Body>>> + Send>>;

    fn authorize(&mut self, req: Request<B>) -> Self::Future {
        let state = self.state.clone();

        Box::pin(async move {
            let cookie_header = req
                .headers()
                .get(http::header::COOKIE)
                .and_then(|v| v.to_str().ok())
                .unwrap_or("");

            let id_token = match parse_cookie(cookie_header, ID_TOKEN_COOKIE) {
                Some(token) => token,
                None => {
                    return Err(Status::unauthenticated("Missing authentication token")
                        .into_http::<()>()
                        .map(|_| Body::empty()));
                }
            };

            match state.firebase_auth.verify_token(&id_token).await {
                Ok(_claims) => Ok(req),
                Err(_) => Err(Status::unauthenticated("Invalid or expired token")
                    .into_http::<()>()
                    .map(|_| Body::empty())),
            }
        })
    }
}
