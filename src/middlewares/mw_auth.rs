
use axum::body::Body;
use axum::http::Request;

use axum::http::Response;
use axum::middleware::Next;
use tower_cookies::Cookies;

use crate::web::AUTH_TOKEN;
use crate::error::{Error, Result};

pub async fn mw_require_auth(
    cookies: Cookies,
    req: Request<Body>,
    next: Next
) -> Result<Response<Body>> {
    println!("->> {:12} - mw_require_auth", "MIDDLEWARE");

    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());
    
    // TODO: Real auth-token parsing & validation.
    auth_token.ok_or(Error::AuthFailNoAuthTokenCookie)?;
    Ok(next.run(req).await)
}
