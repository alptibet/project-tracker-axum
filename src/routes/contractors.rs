use axum::{extract::State, Json};
use serde_json::{json, Value};

use crate::appstate::AppState;

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

pub async fn api_hello(State(state): State<AppState>) -> Json<Value> {
    println!("{:?}", state);
    Json(json!({
        "name":"Alp"
    }))
}
