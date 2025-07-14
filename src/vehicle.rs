use axum::{debug_handler, Json};




#[derive(Debug, serde::Serialize)]
pub struct Vehicle {
    id: Option<String>,
    manufacturer: String,
    model: String,
    year: u16,
}

#[debug_handler]
pub async fn vehicle_get() -> Json<Vehicle> {
    println!("Caller retrieved a vehicle");
    Json::from(
        Vehicle {
            id: Some(uuid::Uuid::new_v4().to_string()),
            manufacturer: "Dodge".to_string(),
            model: "RAM 1560".to_string(),
            year: 2020,
        }
    )
}


pub async fn vehicle_post() -> &'static str {
    "Vehicle POST endpoint"
}


pub async fn vehicle_put() -> &'static str {
    "Vehicle PUT endpoint"
}