use axum::{extract::State, Json};
use serde_json::{json, Value};

use crate::appstate::AppState;

pub async fn api_hello(State(state): State<AppState>) -> Json<Value> {
    println!("{:?}", state.db);
    Json(json!({
        "name":"Alp"
    }))
}
