use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Systems {
    systems: Vec<SysWithScope>,
}

#[derive(Deserialize, Serialize)]
pub struct SystemDocument {
    _id: ObjectId,
    name: String,
}

#[derive(Deserialize, Serialize)]
pub struct System {
    _id: String,
    name: String,
}

#[derive(Deserialize, Serialize)]
pub enum Scope {
    Design,
    Installation,
    Commissioning,
}

#[derive(Deserialize, Serialize)]
struct SysWithScope {
    system: System,
    scope: Scope,
}
