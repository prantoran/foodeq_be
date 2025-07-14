use axum::{debug_handler, Json, extract::Query};


#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Vehicle {
    manufacturer: String,
    model: String,
    year: u16,
    id: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Customer {
    first_name: String,
    last_name: String,
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


pub async fn vehicle_post2(
    Query(mut v): Query<Vehicle>,
    Query(mut customer): Query<Customer>
) -> Json<Vehicle> {
    println!("Customer: {0} {1}", customer.first_name, customer.last_name);
    v.id = Some(uuid::Uuid::new_v4().to_string());
    Json::from(v)
}


pub async fn vehicle_put() -> &'static str {
    "Vehicle PUT endpoint"
}