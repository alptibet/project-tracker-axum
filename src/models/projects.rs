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

#[derive(Deserialize, Serialize)]
pub enum Scope {
    Design(String),
    Installation(String),
    Commissioning(String),
}

#[derive(Deserialize, Serialize)]
pub enum SystemName {
    Fire(String),
    Public(String),
    Hvac(String),
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Clone)]
pub struct Material {
    pub partNumber: String,
    pub brand: String,
    pub qty: i32,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct SystemWithMaterials {
    pub name: String,
    pub scope: Vec<String>,
    pub materials: Vec<Material>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SystemWithoutMaterials {
    pub name: String,
    pub scope: Vec<String>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct ProjectDocumentWithMaterials {
    pub _id: ObjectId,
    pub name: String,
    pub address: String,
    pub active: bool,
    pub completed: bool,
    pub duration: i32,
    pub startDate: DateTime,
    pub completionDate: DateTime,
    pub contractor: String,
    pub systems: Vec<SystemWithMaterials>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct ProjectDocumentWithoutMaterials {
    pub _id: ObjectId,
    pub name: String,
    pub address: String,
    pub active: bool,
    pub completed: bool,
    pub duration: i32,
    pub startDate: DateTime,
    pub completionDate: DateTime,
    pub contractor: String,
    pub systems: Vec<SystemWithoutMaterials>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct ProjectWithMaterials {
    pub _id: String,
    pub name: String,
    pub address: String,
    pub active: bool,
    pub completed: bool,
    pub duration: i32,
    pub startDate: String,
    pub completionDate: String,
    pub contractor: String,
    pub systems: Vec<SystemWithMaterials>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct ProjectWithoutMaterials {
    pub _id: String,
    pub name: String,
    pub address: String,
    pub active: bool,
    pub completed: bool,
    pub duration: i32,
    pub startDate: String,
    pub completionDate: String,
    pub contractor: String,
    pub systems: Vec<SystemWithoutMaterials>,
}

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
#[derive(Deserialize, Serialize, Validate, Debug)]
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
    pub contractor: Option<String>,
    pub systems: Vec<SystemWithoutMaterials>,
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
