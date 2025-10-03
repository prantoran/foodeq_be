use crate::{error::{Error, Result}, web::AUTH_TOKEN};

use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};
use tracing::debug;

pub fn routes() -> Router {
    Router::new()
        .route("/api/login", post(api_login))
        // Add other routes here as needed
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

#[axum::debug_handler]
async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    debug!("{:<12} - api_login with payload: {:?}", "HANDLER", payload);

    // TODO: Implement real db/auth logic.
    if payload.username != "demo" || payload.password != "123" {
        return Err(Error::LoginFail);
    }

    // FIXME: Implement real auth-token generatution/signature.
    cookies.add(Cookie::new(AUTH_TOKEN, "user-1.exp_date.signature"));
    debug!("{:<12} - Login successful for user: {}", "HANDLER", payload.username);

    // Create the success body.
    let body = Json(json!({
        "result": {
            "success": true,
        },
    }));

    Ok(body)
}



