use axum::{
    async_trait,
    body::HttpBody,
    extract::{FromRequest, Json},
    http::Request,
    http::StatusCode,
    BoxError, RequestExt,
};
use bson::Document;
use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use validator::Validate;

use super::systems::{Systems, SystemsDocument};

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
pub struct ProjectDocument {
    pub _id: ObjectId,
    pub name: String,
    pub address: String,
    pub active: bool,
    pub completed: bool,
    pub duration: i32,
    pub startDate: DateTime,
    pub completionDate: DateTime,
    pub contractor: ObjectId,
    pub systems: Vec<SystemsDocument>,
}

// -> I ha to create this struct to use documets returned from pipelines, since the pipeline operations return documents for referenced fields.
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
pub struct ProjectDocumentFind {
    pub _id: ObjectId,
    pub name: String,
    pub address: String,
    pub active: bool,
    pub completed: bool,
    pub duration: i32,
    pub startDate: DateTime,
    pub completionDate: DateTime,
    pub contractor: Document,
    pub systems: Vec<Document>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
pub struct Project {
    pub _id: String,
    pub name: String,
    pub address: String,
    pub active: bool,
    pub completed: bool,
    pub duration: i32,
    pub startDate: String,
    pub completionDate: String,
    pub contractor: String,
    pub systems: Vec<Systems>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Validate)]
pub struct ProjectInput {
    #[validate(length(min = 3, message = "Name must be at least 3 characters long"))]
    pub name: String,
    pub address: String,
    pub active: bool,
    pub completed: bool,
    pub duration: i32,
    pub startDate: String,
    pub completionDate: String,
    #[validate(required(message = "Project must have a contractor"))]
    pub contractor: Option<ObjectId>,
    pub systems: Vec<SystemsDocument>,
}

#[async_trait]
impl<S, B> FromRequest<S, B> for ProjectInput
where
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<Value>);
    async fn from_request(req: Request<B>, _state: &S) -> Result<Self, Self::Rejection> {
        let Json(project) = req.extract::<Json<ProjectInput>, _>().await.unwrap();
        if let Err(errors) = project.validate() {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "message":"validation error",
                    "errors": errors
                })),
            ));
        }
        Ok(project)
    }
}
