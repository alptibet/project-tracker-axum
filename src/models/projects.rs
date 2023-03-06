use mongodb::bson::{oid::ObjectId, DateTime, Document};
use serde::{Deserialize, Serialize};

use super::systems::SysDetails;

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
pub struct ProjectDocument {
    pub _id: ObjectId,
    pub name: String,
    pub address: String,
    pub active: bool,
    pub completed: bool,
    pub duration: i32,
    pub startDate: DateTime,
    pub completionDate: DateTime,
    pub contractor: Document,
    pub systems: Vec<Document>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
pub struct Project {
    pub _id: String,
    pub name: String,
    pub address: String,
    pub active: bool,
    pub completed: bool,
    pub duration: i32,
    pub startDate: String,
    pub completionDate: String,
    pub contractor: String,
    pub systems: Vec<SysDetails>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
pub struct ProjectInput {
    pub name: String,
    pub address: String,
    pub active: bool,
    pub completed: bool,
    pub duration: i32,
    pub startDate: String,
    pub completionDate: String,
    pub contractor: ObjectId,
    pub systems: Vec<SysDetails>,
}
