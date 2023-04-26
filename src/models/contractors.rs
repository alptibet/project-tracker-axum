use axum::{
    async_trait,
    body::HttpBody,
    extract::{FromRequest, Json},
    http::Request,
    http::StatusCode,
    BoxError, RequestExt,
};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use validator::Validate;

#[derive(Deserialize, Serialize)]
pub struct Contractor {
    pub _id: String,
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct ContractorDocument {
    pub _id: ObjectId,
    pub name: String,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct ContractorInput {
    #[validate(length(
        min = 2,
        message = "Contractor name must be at least 2 characters long"
    ))]
    pub name: String,
}

#[async_trait]
impl<S, B> FromRequest<S, B> for ContractorInput
where
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<Value>);
    async fn from_request(req: Request<B>, _state: &S) -> Result<Self, Self::Rejection> {
        let Json(contractor) = req.extract::<Json<ContractorInput>, _>().await.unwrap();
        if let Err(errors) = contractor.validate() {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "status":"validation error",
                    "errors": errors
                })),
            ));
        }
        Ok(contractor)
    }
}
