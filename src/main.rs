use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, get_service, post}, 
    Router
};
use serde_json::json;
use tower_http::services::ServeDir;
use std::env;

mod vehicle;    
mod hello;

mod error;

use vehicle::{vehicle_get, vehicle_post, vehicle_put, vehicle_post2};


// AppState holds the Gemini API client or any other shared state.
#[derive(Clone)]
struct AppState {
    gemini_api_key: String,
}


// The request body for the /analyze-image endpoint.
// It expects a single field `image` containing the base64-encoded image data.
#[derive(serde::Deserialize)]
struct ImageRequest {
    image: String,
}

// Represents a single food item identified in the image.
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct FoodItem {
    name: String,
    calories: f32,
    protein_g: f32,
    fat_g: f32,
    carbohydrates_g: f32,
    sugar_g: f32,
    sodium_mg: f32,
}

// The final JSON response structure sent back to the client.
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct NutritionResponse {
    foods: Vec<FoodItem>,
}

// Handler for the /analyze-image endpoint
async fn analyze_image(
    State(state): State<AppState>,
    Json(payload): Json<ImageRequest>,
) -> Result<Json<NutritionResponse>, StatusCode> {
    match call_gemini_api(&state.gemini_api_key, &payload.image).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => {
            eprintln!("Error calling Gemini API: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Function to call Gemini 2.5 Pro API for nutritional analysis
async fn call_gemini_api(api_key: &str, base64_image: &str) -> Result<NutritionResponse, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    
    let prompt = "Analyze this food image and provide detailed nutritional information. For each food item visible, provide the name, estimated calories, protein (g), fat (g), carbohydrates (g), sugar (g), and sodium (mg). Return the response as a JSON object with a 'foods' array containing objects with these exact fields: name, calories, protein_g, fat_g, carbohydrates_g, sugar_g, sodium_mg. Only return the JSON, no additional text.";
    
    let request_body = json!({
        "contents": [{
            "parts": [
                {
                    "text": prompt
                },
                {
                    "inline_data": {
                        "mime_type": "image/jpeg",
                        "data": base64_image
                    }
                }
            ]
        }]
    });


    let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent?key={}", api_key);
    
    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await?;

    let response_text = response.text().await?;

    
    let gemini_response: serde_json::Value = serde_json::from_str(&response_text)?;
    
    // Extract the generated text from Gemini's response
    let generated_text = gemini_response
        .get("candidates")
        .and_then(|c| c.get(0))
        .and_then(|c| c.get("content"))
        .and_then(|c| c.get("parts"))
        .and_then(|p| p.get(0))
        .and_then(|p| p.get("text"))
        .and_then(|t| t.as_str())
        .ok_or("Failed to extract text from Gemini response")?;

    println!("generated_text: {}\n", generated_text);

    // Parse the JSON response from Gemini with error handling
    let nutrition_response = parse_gemini_response(generated_text)?;
    
    Ok(nutrition_response)
}

// Robust parser for Gemini response that handles missing fields and unknown keys
fn parse_gemini_response(generated_text: &str) -> Result<NutritionResponse, Box<dyn std::error::Error>> {
    // First, try to parse as JSON
    let json_value: serde_json::Value = match serde_json::from_str(generated_text) {
        Ok(value) => value,
        Err(_) => {
            // If direct parsing fails, try to extract JSON from the text
            // Sometimes Gemini wraps JSON in markdown code blocks or adds extra text
            println!("Could not parse directly\n");
            if let Some(json_str) = extract_json_from_text(generated_text) {
                println!("json_str: {}", json_str);
                serde_json::from_str(&json_str)?
            } else {
                return Err("No valid JSON found in Gemini response".into());
            }
        }
    };

    // Extract foods array
    let foods_array = json_value
        .get("foods")
        .and_then(|f| f.as_array())
        .ok_or("No 'foods' array found in response")?;

    let mut foods = Vec::new();

    for food_value in foods_array {
        let food_item = parse_food_item(food_value)?;
        foods.push(food_item);
    }

    Ok(NutritionResponse { foods })
}

// Extract JSON from text that might contain markdown or extra content
fn extract_json_from_text(text: &str) -> Option<String> {
    // Look for JSON wrapped in code blocks
    if let Some(start) = text.find("```json") {
        let json_start = start + 7; // Skip "```json"
        if let Some(end) = text[json_start..].find("```") {
            let json_end = json_start + end;
            let s = text[json_start..json_end].trim().to_string();
            return Some(s);
        }
    }
    
    // Look for JSON wrapped in regular code blocks
    if let Some(start) = text.find("```") {
        if let Some(end) = text[start + 3..].find("```") {
            let json_start = start + 3;
            let json_end = start + 3 + end;
            let potential_json = text[json_start..json_end].trim();
            if potential_json.starts_with('{') && potential_json.ends_with('}') {
                return Some(potential_json.to_string());
            }
        }
    }
    
    // Look for JSON object in the text
    if let Some(start) = text.find('{') {
        if let Some(end) = text.rfind('}') {
            if end > start {
                return Some(text[start..=end].to_string());
            }
        }
    }
    
    None
}

// Parse individual food item with default values for missing fields
fn parse_food_item(food_value: &serde_json::Value) -> Result<FoodItem, Box<dyn std::error::Error>> {
    let name = food_value
        .get("name")
        .and_then(|n| n.as_str())
        .unwrap_or("Unknown Food")
        .to_string();

    let calories = parse_numeric_field(food_value, "calories").unwrap_or(0.0);
    let protein_g = parse_numeric_field(food_value, "protein_g")
        .or_else(|| parse_numeric_field(food_value, "protein"))
        .unwrap_or(0.0);
    let fat_g = parse_numeric_field(food_value, "fat_g")
        .or_else(|| parse_numeric_field(food_value, "fat"))
        .unwrap_or(0.0);
    let carbohydrates_g = parse_numeric_field(food_value, "carbohydrates_g")
        .or_else(|| parse_numeric_field(food_value, "carbohydrates"))
        .or_else(|| parse_numeric_field(food_value, "carbs"))
        .unwrap_or(0.0);
    let sugar_g = parse_numeric_field(food_value, "sugar_g")
        .or_else(|| parse_numeric_field(food_value, "sugar"))
        .unwrap_or(0.0);
    let sodium_mg = parse_numeric_field(food_value, "sodium_mg")
        .or_else(|| parse_numeric_field(food_value, "sodium"))
        .unwrap_or(0.0);

    Ok(FoodItem {
        name,
        calories,
        protein_g,
        fat_g,
        carbohydrates_g,
        sugar_g,
        sodium_mg,
    })
}

// Helper function to parse numeric fields that might be strings or numbers
fn parse_numeric_field(value: &serde_json::Value, field_name: &str) -> Option<f32> {
    value.get(field_name).and_then(|v| {
        match v {
            serde_json::Value::Number(n) => n.as_f64().map(|f| f as f32),
            serde_json::Value::String(s) => {
                // Try to parse string as number, removing any units
                let cleaned = s.trim()
                    .replace("g", "")
                    .replace("mg", "")
                    .replace("kcal", "")
                    .replace("cal", "")
                    .trim()
                    .to_string();
                cleaned.parse::<f32>().ok()
            }
            _ => None,
        }
    })
}

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenv::dotenv().ok();
    
    // Get the Gemini API key from environment
    let gemini_api_key = env::var("GEMINI_API_KEY")
        .expect("GEMINI_API_KEY must be set in .env file");
    // Create app state with the API key
    let app_state = AppState {
        gemini_api_key,
    };

    println!("Starting nutrition analysis server...");

    // build our application with routes
    let routes_all: Router = Router::new().route("/", get(|| async { "Hello, World!" }));

    let router01 = Router::new()
        .route("/vehicle", post(vehicle_post).get(vehicle_get).put(vehicle_put));
    let router02: Router = Router::new()
        .route("/vehicle2", post(vehicle_post2));
    
    // Add the nutrition analysis endpoint
    let nutrition_router = Router::new()
        .route("/analyze-image", post(analyze_image))
        .with_state(app_state);

    let routes_all = routes_all.merge(hello::routes_hello());
    let routes_all = routes_all.merge(router01);
    let routes_all = routes_all.merge(router02);
    let routes_all = routes_all.merge(nutrition_router);
    let routes_all = routes_all.fallback_service(routes_static());

    // run our app with hyper, listening globally on port 3000
    let addr: &str = "0.0.0.0:3000";
    println!("Server running on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, routes_all).await.unwrap();
}

fn routes_static() -> Router {
    Router::new()
        .nest_service("/pub", get_service(ServeDir::new("./public")))
}
