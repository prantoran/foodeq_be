
use axum::body::Body;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::Request;

use axum::http::Response;
use axum::middleware::Next;
use axum::RequestPartsExt;
use lazy_regex::regex_captures;
use tower_cookies::Cookies;

use crate::error;
use crate::web::AUTH_TOKEN;
use crate::error::{Error, Result};
use crate::ctx::Ctx;

pub async fn mw_require_auth(
    // cookies: Cookies,
    ctx: Result<Ctx>,
    req: Request<Body>,
    next: Next
) -> Result<Response<Body>> {
    println!("->> {:12} - mw_require_auth - {ctx:?}", "MIDDLEWARE");

    ctx?;

    Ok(next.run(req).await)
}

// Ctx Extractor

// Implement async trait
impl<S> FromRequestParts<S> for Ctx 
where
    S: Send + Sync, // Required by async_trait
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        println!("->> {:12} - Ctx", "EXTRACTOR");

        // User the cookies extractor.
        let cookies = parts.extract::<Cookies>()
            .await.unwrap();

        let auth_token = cookies.get(AUTH_TOKEN)
            .map(|c| c.value().to_string());

        // Parse token.
        let (user_id, _exp, _sign) = auth_token
            .ok_or(Error::AuthFailNoAuthTokenCookie)
            .and_then(|token| parse_token(token))?;

        // TODO: Token components validation
        println!("->> {:12} - Ctx - user_id: {}", "EXTRACTOR", user_id);
        Ok(Ctx::new(user_id))
    } 
}


// END - Ctx Extractor

/// Parse a token of format `user-[user-id].[expiration].[signature]`
/// Returns (user_id, expiration, signature)
pub fn parse_token(token: String) -> Result<(u64, String, String)> {
    println!("->> {:<12} - parse_token - token: {token}", "PARSE_TOKEN");
    
    let (_whole, user_id, exp, sign) = regex_captures!(
        r#"^user-(\d+)\.(\d+)\.(.+)$"#, // a literal regex
        &token
    ).ok_or(Error::AuthFailTokenWrongFormat)?;

    let user_id: u64 = user_id
        .parse()
        .map_err(|_| Error::AuthFailTokenWrongFormat)?;

    Ok((user_id, exp.to_string(), sign.to_string()))
}
