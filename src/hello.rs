
use axum::{
    response::{Html, IntoResponse},
    extract::{Path, Query},
    Router,
    routing::get,
};
use tracing::debug;

pub fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/{name}", get(handler_hello2)) // mapping by order, name does not matter

}

#[derive(Debug, serde::Deserialize)]
pub struct HelloParams {
    name: Option<String>,
}

// /hello?name=pinku
pub async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    debug!("{:<12} - handler hello - {params:?}", "HANDLER");
    
    let name: &str = params.name.as_deref().unwrap_or("World!!!");
    
    Html(format!("<h1>Hello, <strong>{name}</strong></h1>"))
}

// /hello/pinku
pub async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    debug!("{:<12} - handler hello2 - {name:?}", "HANDLER");
    
    Html(format!("<h1>Hello, <strong>{name}</strong></h1>"))
}