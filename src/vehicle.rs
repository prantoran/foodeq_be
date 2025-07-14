use axum::{debug_handler, Json};


#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Vehicle {
    manufacturer: String,
    model: String,
    year: u16,
    id: Option<String>,
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


pub async fn vehicle_post(Json(mut v): Json<Vehicle>) -> Json<Vehicle> {
    println!("Manufacturer: {0}, Model: {1}, Year: {2}", v.manufacturer, v.model, v.year);
    v.id = Some(uuid::Uuid::new_v4().to_string());
    Json::from(v)
}


pub async fn vehicle_put() -> &'static str {
    "Vehicle PUT endpoint"
}