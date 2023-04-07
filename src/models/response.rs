use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[derive(Serialize, Deserialize)]
pub struct VecResponse<DocType> {
    pub status: String,
    pub data: Vec<DocType>,
}

#[derive(Serialize, Deserialize)]
pub struct DocResponse<DocType> {
    pub status: String,
    pub data: DocType,
}

#[typeshare]
#[derive(Serialize, Deserialize)]
pub struct MessageResponse {
    pub status: String,
}
