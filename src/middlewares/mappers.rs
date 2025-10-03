
use axum::{
    http::{Response, Uri}, response::IntoResponse, Json
};
use reqwest::Method;
use serde_json::json;
use tracing::debug;
use uuid::Uuid;

use crate::{ctx::Ctx, error::{Result, Error}, log::log_request};

pub async fn mw_response_map(
    ctx: Result<Ctx>,
    uri: Uri,
    req_method: Method,
    res: Response<axum::body::Body>
) -> Response<axum::body::Body> {
    debug!("{:<12} - mw_response_map", "MIDDLEWARE");
    // let ctx = ctx.map(|ctx| ctx.0).ok();
    let uuid = Uuid::new_v4();

    // -- Get the eventual response error.
    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|se| se.client_status_and_error());
    
    // --If client error, build the new response.
    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
                "error": {
                    "type": client_error.as_ref(),
                    "req_uuid": uuid.to_string(),
                }
            });
 
            debug!("{:<12} - {client_error_body}", "CLIENT_ERROR");
            
            // Build the new response from the client error body.
            (*status_code, Json(client_error_body)).into_response()
        });
    
    // -- TODO: Build and log the server log line.
    let client_error = client_status_error.unzip().1;
    let _ = log_request(uuid, req_method, uri, ctx, service_error, client_error).await;

    // debug!("server log line - {uuid} - Error: {service_error:?} Client Error: {client_error:?}");
    debug!("\n"); // will not have any meaning when multiple requests are logged at the same time.
    error_response.unwrap_or(res)
}