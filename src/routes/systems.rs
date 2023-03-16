use crate::appstate::AppState;
use crate::controllers::contractors;
use crate::errors::AppError;
use crate::models::contractors::{Contractor, ContractorInput};
use crate::models::response::{DocResponse, MessageResponse, VecResponse};
use crate::utils::parse_oid;
use axum::extract::{Json, Path, State};

pub async fn get_all_systems(State(state):State<AppState>) -> Result<Json<VecResponse<System>>, AppError> {
    todo!();
}
