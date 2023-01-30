use crate::models::users::{User, UserDocument, UserRole};
use axum::Json;
use futures::TryStreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, Document};
use mongodb::options::FindOneAndUpdateOptions;
use mongodb::options::ReturnDocument;
use mongodb::Database;

pub async fn get_all(db: &Database) -> mongodb::error::Result<Vec<User>>{
    let collection = db.collection::<UserDocument>("users");
    let mut cursor = collection.find(None, None).await?;
    let mut users: Vec<User> = vec![];

    while let Some(result) = cursor.try_next().await? {
        let _id = result._id;
        let name = result.name;
        let surname = result.surname;
        let username = result.username;
        let email = result.email;
        let active = result.active;
        let password = result.password;
        let password_change_at = result.passwordChangeAt;
        let userrole = result.role;
        let user_json = User {
            _id: _id.to_string(),
            name,
            surname,
            username,
            email,
            active: active.to_string(),
            password,
            passwordChangeAt: password_change_at.to_string(),
            role: match userrole {
                UserRole::Admin => "admin".to_string(),
                UserRole::User => "user".to_string(),
                UserRole::Superuser => "superuser".to_string(),
            },
        };
        users.push(user_json);
    }

    Ok(users)
}
