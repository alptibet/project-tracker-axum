use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct EquipmentDocument {
    pub _id: ObjectId,
    pub brand: String,
    pub partNo: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct Equipment {
    pub _id: String,
    pub brand: String,
    pub partNo: String,
}
