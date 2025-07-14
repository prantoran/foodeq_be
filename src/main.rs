use axum::{
    routing::get,
    Router,
};


#[tokio::main]
async fn main() {
    /*
    
    There are othere ways to set up a Tokio runtime, such as using `tokio::runtime::Builder` to customize the runtime configuration.    
    However, the `#[tokio::main]` macro is the most common and convenient way
     */

    println!("Hello, world!");

    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
