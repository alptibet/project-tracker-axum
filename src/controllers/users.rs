use crate::models::auth::UserInput;
use crate::models::users::{User, UserDocument, UserRole};
use axum::Json;
use bcrypt::hash;
use futures::TryStreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, DateTime, Document};
use mongodb::options::FindOneAndUpdateOptions;
use mongodb::options::ReturnDocument;
use mongodb::Database;

pub async fn get_all(db: &Database) -> mongodb::error::Result<Vec<User>> {
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
                UserRole::Admin => "Admin".to_string(),
                UserRole::User => "User".to_string(),
                UserRole::Superuser => "Superuser".to_string(),
            },
        };
        users.push(user_json);
    }
    Ok(users)
}

pub async fn find_one(db: &Database, oid: ObjectId) -> mongodb::error::Result<Option<User>> {
    let collection = db.collection::<UserDocument>("users");

    let user_doc = collection.find_one(doc! {"_id":oid}, None).await?;
    if user_doc.is_none() {
        return Ok(None);
    }

    let unwrapped_doc = user_doc.unwrap();
    let user_json = User {
        _id: unwrapped_doc._id.to_string(),
        username: unwrapped_doc.username,
        name: unwrapped_doc.name,
        surname: unwrapped_doc.surname,
        email: unwrapped_doc.email,
        password: unwrapped_doc.password,
        active: unwrapped_doc.active.to_string(),
        passwordChangeAt: unwrapped_doc.passwordChangeAt.to_string(),
        role: match unwrapped_doc.role {
            UserRole::Admin => "Admin".to_string(),
            UserRole::User => "User".to_string(),
            UserRole::Superuser => "Superuser".to_string(),
        },
    };

    Ok(Some(user_json))
}

pub async fn insert_one(db: &Database, input: Json<UserInput>) -> mongodb::error::Result<User> {
    let collection = db.collection::<Document>("users");
    let password_created_at: DateTime = DateTime::now();
    let hashed_password = hash(&input.password, 12).unwrap();
    let user_document = doc! {
        "name": &input.name,
        "surname": &input.surname,
        "username": &input.username,
        "email": &input.email,
        "active": true,
        "password": &hashed_password,
        "passwordChangeAt": password_created_at,
        "role": "User".to_string(),
    };

    let insert_one_result = collection.insert_one(user_document, None).await?;
    let name = &input.name;
    let surname = &input.surname;
    let username = &input.username;
    let email = &input.email;
    let active = true;
    let password = hashed_password;
    let role = "User".to_string();
    let user_json = User {
        _id: insert_one_result.inserted_id.to_string(),
        name: name.to_string(),
        surname: surname.to_string(),
        username: username.to_string(),
        email: email.to_string(),
        active: active.to_string(),
        password,
        passwordChangeAt: password_created_at.to_string(),
        role,
    };
    Ok(user_json)
}

pub async fn deactivate_user(db: &Database, oid: ObjectId) -> mongodb::error::Result<Option<User>> {
    let collection = db.collection::<UserDocument>("users");
    let update_options = FindOneAndUpdateOptions::builder()
        .return_document(ReturnDocument::After)
        .build();
    let user_doc = collection
        .find_one_and_update(
            doc! {"_id":oid},
            doc! {"$set": {"active": false}},
            update_options,
        )
        .await?;
    if user_doc.is_none() {
        return Ok(None);
    }
    let unwrapped_doc = user_doc.unwrap();
    let user_json = User {
        _id: unwrapped_doc._id.to_string(),
        username: unwrapped_doc.username,
        name: unwrapped_doc.name,
        surname: unwrapped_doc.surname,
        email: unwrapped_doc.email,
        password: unwrapped_doc.password,
        active: unwrapped_doc.active.to_string(),
        passwordChangeAt: unwrapped_doc.passwordChangeAt.to_string(),
        role: match unwrapped_doc.role {
            UserRole::Admin => "Admin".to_string(),
            UserRole::User => "User".to_string(),
            UserRole::Superuser => "Superuser".to_string(),
        },
    };

    Ok(Some(user_json))
}
