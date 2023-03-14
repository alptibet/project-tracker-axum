use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct VecResponse<DocType> {
    pub status: String,
    pub data: Vec<DocType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocResponse<DocType> {
    pub status: String,
    pub data: DocType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageResponse {
    pub status: String,
}
