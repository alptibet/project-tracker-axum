use axum::Json;
use futures::TryStreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, Document};
use mongodb::options::FindOneAndUpdateOptions;
use mongodb::options::ReturnDocument;
use mongodb::Database;

use crate::models::materials::{Material, MaterialDocument, MaterialInput};

pub async fn get_all(db: &Database) -> mongodb::error::Result<Vec<Material>> {
    let collection = db.collection::<MaterialDocument>("materials");
    let mut cursor = collection.find(None, None).await?;
    let mut materials: Vec<Material> = vec![];

    while let Some(result) = cursor.try_next().await? {
        let materials_json = Material {
            _id: result._id.to_string(),
            brand: result.brand,
            partNumber: result.partNumber,
            description: result.description,
        };
        materials.push(materials_json);
    }
    Ok(materials)
}

pub async fn get_one(db: &Database, oid: ObjectId) -> mongodb::error::Result<Option<Material>> {
    let collection = db.collection::<MaterialDocument>("materials");

    let material_doc = collection.find_one(doc! {"_id": oid}, None).await?;

    if material_doc.is_none() {
        return Ok(None);
    }

    let unwrapped_doc = material_doc.unwrap();
    let material_json = Material {
        _id: unwrapped_doc._id.to_string(),
        brand: unwrapped_doc.brand,
        partNumber: unwrapped_doc.partNumber,
        description: unwrapped_doc.description,
    };

    Ok(Some(material_json))
}

pub async fn insert_one(
    db: &Database,
    input: Json<MaterialInput>,
) -> mongodb::error::Result<Material> {
    let collection = db.collection::<Document>("materials");
    let material_document = doc! {"brand": &input.brand, "partNumber": &input.partNumber, "description": &input.description};
    let insert_one_result = collection.insert_one(material_document, None).await?;

    let material_json = Material {
        _id: insert_one_result
            .inserted_id
            .as_object_id()
            .unwrap()
            .to_string(),
        brand: input.brand.clone().unwrap(),
        partNumber: input.partNumber.clone().unwrap(),
        description: input.description.to_string(),
    };
    Ok(material_json)
}

pub async fn delete_one(db: &Database, oid: ObjectId) -> mongodb::error::Result<Option<String>> {
    let collection = db.collection::<Document>("materials");
    let material_doc = collection
        .find_one_and_delete(doc! {"_id": oid}, None)
        .await?;
    if material_doc.is_none() {
        return Ok(None);
    }
    Ok(Some("Document deleted".to_string()))
}

pub async fn update_one(
    db: &Database,
    oid: ObjectId,
    input: Json<MaterialInput>,
) -> mongodb::error::Result<Option<Material>> {
    let collection = db.collection::<MaterialDocument>("materials");
    let update_options = FindOneAndUpdateOptions::builder()
        .return_document(ReturnDocument::After)
        .build();

    let material_doc = collection
        .find_one_and_update(
            doc! {"_id": oid},
            doc! {"$set": {"brand": &input.brand, "partNumber": &input.partNumber, "description": &input.description}},
            update_options,
        )
        .await?;
    if material_doc.is_none() {
        return Ok(None);
    }

    let unwrapped_doc = material_doc.unwrap();
    let material_json = Material {
        _id: unwrapped_doc._id.to_string(),
        brand: unwrapped_doc.brand,
        partNumber: unwrapped_doc.partNumber,
        description: unwrapped_doc.description,
    };

    Ok(Some(material_json))
}
