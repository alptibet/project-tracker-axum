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
pub struct SystemDocument {
    pub _id: ObjectId,
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct System {
    pub _id: String,
    pub name: String,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct SystemInput {
    #[validate(length(min = 3, message = "System name must be at least 3 characters long"))]
    pub name: String,
}

#[async_trait]
impl<S, B> FromRequest<S, B> for SystemInput
where
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<Value>);
    async fn from_request(req: Request<B>, _state: &S) -> Result<Self, Self::Rejection> {
        let Json(system) = req.extract::<Json<SystemInput>, _>().await.unwrap();
        if let Err(errors) = system.validate() {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "status":"validation error",
                    "errors": errors
                })),
            ));
        }
        Ok(system)
    }
}
