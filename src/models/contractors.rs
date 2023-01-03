use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

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

#[derive(Deserialize, Serialize)]
pub struct ContractorInput {
    pub name: String,
}
