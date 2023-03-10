use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

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

//Still could not figure out how to use if input is not of any of these
#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum Scope {
    Design,
    Installation,
    Commissioning,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Systems {
    //was SysDetails
    pub system: String,
    pub scope: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SystemsDocument {
    //was SysDetailsDocument
    pub system: ObjectId,
    pub scope: Scope,
}
