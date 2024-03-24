use axum::{extract::{Request, State}, middleware::Next, response::{IntoResponse, Response}};
use axum_extra::{headers::{authorization::Bearer, Authorization}, TypedHeader};

use crate::{auth, AppState};

#[derive(Debug, Clone)]
pub struct Identity {
    pub username: String,
    pub user_id: u64,
}

/// Decodes Authorization header and puts it into identity extension
/// 
/// As this is a relaxed auth middleware, the authorization is optional
pub async fn relaxed_auth_middleware(
    State(state): State<AppState>,
    authorization: Option<TypedHeader<Authorization<Bearer>>>,
    mut req: Request,
    next: Next
) -> Response {
    // decode authorization header and create identity
    let identity = if let Some(TypedHeader(Authorization(bearer))) = authorization {
        let decoded_token = match auth::validate_access_token(bearer.token(), &state.settings.token_secret) {
            Ok(token) => token,
            Err(err) => return err.into_response(),
        };
        Some(Identity{
            username: decoded_token.claims.sub,
            user_id: decoded_token.claims.sub_id,
        })
    } else {
        None
    };

    req.extensions_mut().insert(identity);

    let resp = next.run(req).await;

    resp
}