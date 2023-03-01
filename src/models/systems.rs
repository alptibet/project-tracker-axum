use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SystemDocument {
    pub _id: ObjectId,
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct System {
    pub _id: String,
    pub name: String,
}
