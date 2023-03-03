use mongodb::bson::{oid::ObjectId, Document};
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
pub struct SysWithScope {
    pub system: SystemDocument,
    pub scope: Scope,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SysWithScope2 {
    pub system: Document,
    pub scope: String,
}
