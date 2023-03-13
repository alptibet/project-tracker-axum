use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct SystemDocument {
    pub _id: ObjectId,
    pub name: String,
    pub scope: Vec<Scope>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct System {
    pub _id: String,
    pub name: String,
    pub scope: Vec<String>,
}

//Still could not figure out how to use if input is not of any of these
#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum Scope {
    Design(String),
    Installation(String),
    Commissioning(String),
}
