use axum::Json;
use futures::TryStreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, Document};
use mongodb::options::FindOneAndUpdateOptions;
use mongodb::options::ReturnDocument;
use mongodb::Database;

use crate::models::systems::{System, SystemDocument, SystemInput};

pub async fn get_all(db: &Database) -> mongodb::error::Result<Vec<System>> {
    let collection = db.collection::<SystemDocument>("systems");
    let mut cursor = collection.find(None, None).await?;
    let mut systems: Vec<System> = vec![];

    while let Some(result) = cursor.try_next().await? {
        let systems_json = System {
            _id: result._id.to_string(),
            name: result.name,
        };
        systems.push(systems_json);
    }
    Ok(systems)
}

pub async fn get_one(db: &Database, oid: ObjectId) -> mongodb::error::Result<Option<System>> {
    let collection = db.collection::<SystemDocument>("systems");

    let system_doc = collection.find_one(doc! {"_id": oid}, None).await?;
    if system_doc.is_none() {
        return Ok(None);
    }

    let unwrapped_doc = system_doc.unwrap();
    let system_json = System {
        _id: unwrapped_doc._id.to_string(),
        name: unwrapped_doc.name,
    };

    Ok(Some(system_json))
}

pub async fn insert_one(db: &Database, input: Json<SystemInput>) -> mongodb::error::Result<System> {
    let collection = db.collection::<Document>("systems");
    let system_document = doc! {"name": &input.name};
    let insert_one_result = collection.insert_one(system_document, None).await?;
    let system_name = &input.name.to_string();
    let system_json = System {
        _id: insert_one_result
            .inserted_id
            .as_object_id()
            .unwrap()
            .to_string(),
        name: system_name.to_string(),
    };
    Ok(system_json)
}

pub async fn delete_one(db: &Database, oid: ObjectId) -> mongodb::error::Result<Option<String>> {
    let collection = db.collection::<Document>("systems");
    let system_doc = collection
        .find_one_and_delete(doc! {"_id": oid}, None)
        .await?;
    if system_doc.is_none() {
        return Ok(None);
    }
    Ok(Some("Document deleted".to_string()))
}

pub async fn update_one(
    db: &Database,
    oid: ObjectId,
    input: Json<SystemInput>,
) -> mongodb::error::Result<Option<System>> {
    let collection = db.collection::<SystemDocument>("systems");
    let update_options = FindOneAndUpdateOptions::builder()
        .return_document(ReturnDocument::After)
        .build();

    let system_doc = collection
        .find_one_and_update(
            doc! {"_id": oid},
            doc! {"$set": {"name": input.name.clone()}},
            update_options,
        )
        .await?;

    if system_doc.is_none() {
        return Ok(None);
    }

    let unwrapped_doc = system_doc.unwrap();
    let system_json = System {
        _id: unwrapped_doc._id.to_string(),
        name: unwrapped_doc.name,
    };

    Ok(Some(system_json))
}
