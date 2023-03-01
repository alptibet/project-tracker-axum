use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

use super::{contractors::Contractor, systems::System};

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct ProjectDocument {
    pub _id: ObjectId,
    pub name: String,
    pub address: String,
    pub active: bool,
    pub completed: bool,
    pub duration: i32,
    pub startDate: DateTime,
    pub contractor: ObjectId,
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
    pub contractor: String,
}
