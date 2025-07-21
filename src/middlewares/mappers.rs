
use axum::{
    http::Response
};

pub async fn main_response_mapper(res: Response<axum::body::Body>) -> Response<axum::body::Body> {
    println!("->> {:<12} - main_response_mapper", "MIDDLEWARE");

    println!();
    res
}