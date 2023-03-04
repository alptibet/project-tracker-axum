use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Systems {
    pub systems: Vec<SysWithScope>,
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

#[derive(Deserialize, Serialize, Debug)]
pub enum Scope {
    Design,
    Installation,
    Commissioning,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SysWithScopeDocument {
    pub system: SystemDocument,
    pub scope: Scope,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SysWithScope {
    pub system: String,
    pub scope: String,
}
