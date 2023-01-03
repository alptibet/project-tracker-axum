use crate::appstate::AppState;
use crate::controllers::contractors;
use crate::models::contractors::Contractor;
use crate::models::response::VecResponse;
use axum::extract::State;
use axum::Json;

pub async fn get_contractors(
    State(state): State<AppState>,
) -> Result<Json<VecResponse<Contractor>>, String> {
    match contractors::find_contractors(state).await {
        Ok(_contractor_doc) => Ok(Json(VecResponse {
            message: "success".to_string(),
            data: _contractor_doc,
        })),
        Err(_error) => Err(_error.to_string()),
    }
}
