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

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct MaterialDocument {
    pub _id: ObjectId,
    pub brand: String,
    pub partNumber: String,
    pub description: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct Material {
    pub _id: String,
    pub brand: String,
    pub partNumber: String,
    pub description: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Validate)]
pub struct MaterialInput {
    #[validate(required(message = "Material must have a brand"))]
    pub brand: Option<String>,
    #[validate(required(message = "Material must have part number"))]
    pub partNumber: Option<String>,
    pub description: String,
}

#[async_trait]
impl<S, B> FromRequest<S, B> for MaterialInput
where
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<Value>);
    async fn from_request(req: Request<B>, _state: &S) -> Result<Self, Self::Rejection> {
        let Json(contractor) = req.extract::<Json<MaterialInput>, _>().await.unwrap();
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
