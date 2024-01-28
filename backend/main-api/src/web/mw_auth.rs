use axum::body::Body;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use tower_cookies::Cookies;
use crate::web::AUTH_TOKEN;
use crate::{Error, Result};
use lazy_regex::regex_captures;

struct AuthToken {
    user_id: u64,
    expiration: String,
    signature: String
}

impl AuthToken {
    // parse token 'user-[user-id].[expiration].[signature]'
    pub fn from_string(token: String) -> Result<AuthToken> {
        let (_whole, uid, exp, sign) = regex_captures!(
            r#"^user-(\d+)\.(.+)\.(.+)"#,
            &token
        ).ok_or(Error::AuthFailTokenWrongFormat)?;

        let user_id = uid.parse().map_err(|_| Error::AuthFailTokenWrongFormat)?;
        let expiration = exp.to_string();
        let signature = sign.to_string();

        Ok(
            AuthToken {
                user_id,
                expiration,
                signature
            }
        )
    }
}

pub async fn mw_require_auth(
    cookies: Cookies,
    req: Request<Body>, 
    next: Next
) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth", "MIDDLEWARE");

    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    // parse
    let token: AuthToken = auth_token
        .ok_or(Error::AuthFailNoAuthToken)
        .and_then(AuthToken::from_string)?;

    // TODO! validate token components.

    Ok(next.run(req).await)
}