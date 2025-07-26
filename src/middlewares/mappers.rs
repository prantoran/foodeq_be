
use axum::{
    http::Response, response::IntoResponse, Json
};
use serde_json::json;
use uuid::Uuid;

use crate::error::Error;

pub async fn main_response_mapper(res: Response<axum::body::Body>) -> Response<axum::body::Body> {
    println!("->> {:<12} - main_response_mapper", "MIDDLEWARE");
    let uuid = Uuid::new_v4();

    // -- Get the eventual response error.
    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|se| se.clien_status_and_error());
    
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

            println!("->> {:<12} - {client_error_body}", "CLIENT_ERROR");

            (*status_code, Json(client_error_body)).into_response()
        });
    
    // -- TODO: Build and log the server log line.
    println!("   ->> server log line - {uuid} - Error: {service_error:?}");
    println!();
    error_response.unwrap_or(res)
}