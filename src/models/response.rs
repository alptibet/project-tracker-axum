use crate::models::contractors::Contractor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct VecResponse<DocType> {
    pub message: String,
    pub data: Vec<DocType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocResponse<DocType> {
    pub message: String,
    pub data: DocType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageResponse {
    pub message: String,
}

#[derive(Serialize, Deserialize)]
enum DocType {
    Contractor(Contractor),
}
