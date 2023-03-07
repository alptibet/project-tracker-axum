use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Systems {
    pub systems: Vec<SysDetails>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SystemDocument {
    pub _id: ObjectId,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct System {
    pub _id: String,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum Scope {
    Design,
    Installation,
    Commissioning,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SysDetails {
    pub system: String,
    pub scope: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SysDetailsInput {
    pub system: ObjectId,
    pub scope: Scope,
}
