use axum::extract::State;
use axum::response::Html;
use axum::{routing::get, Json, Router};
use serde_json::{json, Value};

use crate::appstate::AppState;
use crate::controllers::contractors;
use crate::errors::AppError;
use crate::models::contractors::Contractor;
use crate::models::response::VecResponse;

pub fn create_routes() -> Router {
    Router::new().route("/contractors", get(handler))
}

// async fn get_contractors(
//     State(state): State<AppState>,
// ) -> Result<Json<VecResponse<Contractor>>, Json<AppError>> {
//     match contractors::find_contractors(state).await {
//         Ok(_contractor_doc) => Ok(Json(VecResponse {
//             message: "success".to_string(),
//             data: _contractor_doc,
//         })),
//         Err(_error) => Err(Json(AppError::build(400))),
//     }
// }

async fn handler() -> Json<Value> {
    Json(json!({
        "name":"Alp"
    }))
}
