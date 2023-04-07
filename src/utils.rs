use crate::errors::AppError;
use mongodb::bson::oid::ObjectId;

pub fn parse_oid(id: String) -> Result<ObjectId, AppError> {
    let oid = ObjectId::parse_str(id);
    match oid {
        Ok(_oid) => Ok(_oid),
        Err(_error) => Err(AppError::OidParseError),
    }
}
