use axum::{
    async_trait,
    body::HttpBody,
    extract::{FromRequest, Json},
    http::Request,
    http::StatusCode,
    BoxError, RequestExt,
};
use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use validator::{Validate, ValidationError};

#[derive(Deserialize, Serialize)]
pub enum Scope {
    Design,
    Installation,
    Commissioning,
    Nothing,
}

#[derive(Deserialize, Serialize)]
pub enum SystemName {
    Fire,
    Public,
    Hvac,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Material {
    pub partNumber: String,
    pub brand: String,
    pub qty: i32,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct MaterialWithSysIndicator {
    pub partNumber: String,
    pub brand: String,
    pub qty: i32,
    pub system: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
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
pub struct ProjectDocumentToDelete {
    pub _id: ObjectId,
    pub name: String,
    pub contractor: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct DeletedProject {
    pub _id: String,
    pub name: String,
    pub contractor: String,
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
#[derive(Deserialize, Serialize)]
pub struct UpdatedMaterialsDocument {
    pub _id: ObjectId,
    pub name: String,
    pub systems: Vec<SystemWithMaterials>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct UpdatedMaterials {
    pub _id: String,
    pub name: String,
    pub systems: Vec<SystemWithMaterials>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Validate)]
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
    #[validate(custom = "validate_system")]
    pub systems: Vec<SystemWithMaterials>,
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

//Could not use Enum variants in this function, why? Ugly
fn validate_system(systems: &[SystemWithMaterials]) -> Result<(), ValidationError> {
    let mut validated: bool = false;
    'outer: for system in systems {
        let result = matches!(system.name.as_str(), "Fire" | "Hvac" | "Public");
        if !result {
            validated = false;
            break;
        }
        validated = true;

        let scope = &system.scope;
        for item in scope {
            let result = matches!(item.as_str(), "Design" | "Installation" | "Commissioning");
            if !result {
                validated = false;
                break 'outer;
            }
            validated = true;
        }
    }

    if validated {
        return Ok(());
    }

    Err(ValidationError::new("DENEME"))
}
