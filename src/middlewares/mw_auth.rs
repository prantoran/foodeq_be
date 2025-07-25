
use axum::body::Body;
use axum::http::Request;

use axum::http::Response;
use axum::middleware::Next;
use lazy_regex::regex_captures;
use tower_cookies::Cookies;

use crate::error;
use crate::web::AUTH_TOKEN;
use crate::error::{Error, Result};

pub async fn mw_require_auth(
    cookies: Cookies,
    req: Request<Body>,
    next: Next
) -> Result<Response<Body>> {
    println!("->> {:12} - mw_require_auth", "MIDDLEWARE");

    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());
    
    // Parse token
    let (user_id, exp, sign) = auth_token
        .ok_or(error::Error::AuthFailNoAuthTokenCookie)
        .and_then(|token| parse_token(token))?;
    
    // TODO: Token components validation

    Ok(next.run(req).await)
}

/// Parse a token of format `user-[user-id].[expiration].[signature]`
/// Returns (user_id, expiration, signature)
pub fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_whole, user_id, exp, sign) = regex_captures!(
        r#"^user-(\d+)\.(\d+)\.(.+)$"#, // a literal regex
        &token
    ).ok_or(Error::AuthFailTokenWrongFormat)?;

    let user_id: u64 = user_id
        .parse()
        .map_err(|_| Error::AuthFailTokenWrongFormat)?;

    Ok((user_id, exp.to_string(), sign.to_string()))
}
