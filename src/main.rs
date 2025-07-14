use axum::{
    routing::{get, post}, Router
};

mod vehicle;    

use vehicle::{vehicle_get, vehicle_post, vehicle_put};


#[tokio::main]
async fn main() {
    /*
    
    There are othere ways to set up a Tokio runtime, such as using `tokio::runtime::Builder` to customize the runtime configuration.    
    However, the `#[tokio::main]` macro is the most common and convenient way
     */

    println!("Hello, world!");

    // build our application with a single route
    let app: Router = Router::new().route("/", get(|| async { "Hello, World!" }));
    
    let router01 = Router::new()
        .route("/vehicle", post(vehicle_post).get(vehicle_get).put(vehicle_put));
    
    let app = app.merge(router01);
    
    // run our app with hyper, listening globally on port 3000
    let address: &str = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
