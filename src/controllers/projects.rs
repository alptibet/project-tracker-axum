use axum::Json;
use chrono::Utc;
use futures::TryStreamExt;
use mongodb::options::{FindOneAndUpdateOptions, ReturnDocument};
use mongodb::Database;
use mongodb::{
    bson::oid::ObjectId,
    bson::{doc, Document},
    options::FindOptions,
};

use crate::models::projects::{
    DeletedProject, MaterialWithSysIndicator, ProjectDocumentToDelete,
    ProjectDocumentWithMaterials, ProjectDocumentWithoutMaterials, ProjectInput,
    ProjectWithMaterials, ProjectWithoutMaterials, UpdatedMaterials, UpdatedMaterialsDocument,
};

pub async fn get_all(db: &Database) -> mongodb::error::Result<Vec<ProjectWithoutMaterials>> {
    let collection = db.collection::<ProjectDocumentWithoutMaterials>("projects");
    let filter = doc! {};
    let options = FindOptions::builder()
        .projection(doc! {"systems.materials":0})
        .build();
    let mut cursor = collection.find(filter, options).await?;
    let mut projects: Vec<ProjectWithoutMaterials> = vec![];

    while let Some(result) = cursor.try_next().await? {
        let projects_json = ProjectWithoutMaterials {
            _id: result._id.to_string(),
            name: result.name,
            address: result.address,
            active: result.active,
            completed: result.completed,
            duration: result.duration,
            startDate: result.startDate.to_string(),
            completionDate: result.completionDate.to_string(),
            contractor: result.contractor,
            systems: result.systems,
        };
        projects.push(projects_json);
    }
    Ok(projects)
}

pub async fn get_one(
    db: &Database,
    oid: ObjectId,
) -> mongodb::error::Result<Option<ProjectWithoutMaterials>> {
    let collection = db.collection::<ProjectDocumentWithoutMaterials>("projects");
    let project_doc = collection.find_one(doc! {"_id": oid}, None).await?;
    if project_doc.is_none() {
        return Ok(None);
    }

    let result = project_doc.unwrap();
    let project_json = ProjectWithoutMaterials {
        _id: result._id.to_string(),
        name: result.name,
        address: result.address,
        active: result.active,
        completed: result.completed,
        duration: result.duration,
        startDate: result.startDate.to_string(),
        completionDate: result.completionDate.to_string(),
        contractor: result.contractor,
        systems: result.systems,
    };

    Ok(Some(project_json))
}

pub async fn get_one_with_materials(
    db: &Database,
    oid: ObjectId,
) -> mongodb::error::Result<Option<ProjectWithMaterials>> {
    let collection = db.collection::<ProjectDocumentWithMaterials>("projects");

    let project_doc = collection.find_one(doc! {"_id": oid}, None).await?;
    if project_doc.is_none() {
        return Ok(None);
    }

    let result = project_doc.unwrap();
    let project_json = ProjectWithMaterials {
        _id: result._id.to_string(),
        name: result.name,
        address: result.address,
        active: result.active,
        completed: result.completed,
        duration: result.duration,
        startDate: result.startDate.to_string(),
        completionDate: result.completionDate.to_string(),
        contractor: result.contractor,
        systems: result.systems,
    };

    Ok(Some(project_json))
}

pub async fn insert_one(
    db: &Database,
    input: Json<ProjectInput>,
) -> mongodb::error::Result<ProjectWithMaterials> {
    let collection = db.collection::<Document>("projects");
    let chrono_dt: chrono::DateTime<Utc> = input.startDate.parse().unwrap();
    let start_date: bson::DateTime = chrono_dt.into();
    let chrono_dt: chrono::DateTime<Utc> = input.completionDate.parse().unwrap();
    let completion_date: bson::DateTime = chrono_dt.into();
    let systems = bson::to_bson(&input.systems)?;
    let project_doc = doc! {"name": &input.name.clone(), "address":&input.address, "active": &input.active, "completed":&input.completed, "duration":&input.duration,"startDate":start_date, "completionDate":completion_date, "contractor": &input.contractor, "systems":systems };

    let result = collection.insert_one(project_doc, None).await?;

    let project_json = ProjectWithMaterials {
        _id: result.inserted_id.as_object_id().unwrap().to_string(),
        name: input.name.to_string(),
        address: input.address.clone(),
        active: input.active,
        completed: input.completed,
        duration: input.duration,
        startDate: input.startDate.to_string(),
        completionDate: input.completionDate.to_string(),
        contractor: input.contractor.clone().unwrap(),
        systems: input.systems.to_vec(),
    };
    Ok(project_json)
}

pub async fn update_one(
    db: &Database,
    oid: ObjectId,
    input: Json<ProjectInput>,
) -> mongodb::error::Result<Option<ProjectWithoutMaterials>> {
    let collection = db.collection::<ProjectDocumentWithoutMaterials>("projects");
    let update_options = FindOneAndUpdateOptions::builder()
        .return_document(ReturnDocument::After)
        .build();

    let chrono_dt: chrono::DateTime<Utc> = input.startDate.parse().unwrap();
    let start_dt: bson::DateTime = chrono_dt.into();
    let chrono_dt: chrono::DateTime<Utc> = input.completionDate.parse().unwrap();
    let completion_dt: bson::DateTime = chrono_dt.into();
    let systems = bson::to_bson(&input.systems).unwrap();

    let project_doc = collection
    .find_one_and_update(
        doc! {"_id":oid},
        doc! {"$set":{"name": &input.name, "address":&input.address, "active": &input.active, "completed": &input.completed,"duration":&input.duration, "startDate": start_dt, "completionDate": completion_dt, "contractor":&input.contractor, "systems":systems}}, update_options).await?;

    if project_doc.is_none() {
        return Ok(None);
    };

    let unwrapped_doc = project_doc.unwrap();
    let project_json = ProjectWithoutMaterials {
        _id: unwrapped_doc._id.to_string(),
        name: unwrapped_doc.name.to_string(),
        address: unwrapped_doc.address.to_string(),
        active: unwrapped_doc.active,
        completed: unwrapped_doc.completed,
        duration: unwrapped_doc.duration,
        startDate: unwrapped_doc.startDate.to_string(),
        completionDate: unwrapped_doc.completionDate.to_string(),
        contractor: unwrapped_doc.contractor,
        systems: unwrapped_doc.systems,
    };
    Ok(Some(project_json))
}

pub async fn delete_one(
    db: &Database,
    oid: ObjectId,
) -> mongodb::error::Result<Option<DeletedProject>> {
    let collection = db.collection::<ProjectDocumentToDelete>("projects");

    let project_doc = collection
        .find_one_and_delete(doc! {"_id": oid}, None)
        .await?;

    if project_doc.is_none() {
        return Ok(None);
    };

    let unwrapped_doc = project_doc.unwrap();

    let project_json = DeletedProject {
        _id: unwrapped_doc._id.to_string(),
        name: unwrapped_doc.name,
        contractor: unwrapped_doc.contractor,
    };
    Ok(Some(project_json))
}

pub async fn insert_material(
    db: &Database,
    oid: ObjectId,
    input: MaterialWithSysIndicator,
) -> mongodb::error::Result<Option<UpdatedMaterials>> {
    let collection = db.collection::<UpdatedMaterialsDocument>("projects");
    let query = doc! {"_id":oid, "systems.name":input.system};
    let update = doc! {"$push":{"systems.$.materials":{"brand":input.brand, "partNumber":input.partNumber, "qty":input.qty}}};

    let update_options = FindOneAndUpdateOptions::builder()
        .return_document(ReturnDocument::After)
        .build();

    let project_doc = collection
        .find_one_and_update(query, update, update_options)
        .await?;

    if project_doc.is_none() {
        return Ok(None);
    };

    let unwrapped_doc = project_doc.unwrap();
    let project_json = UpdatedMaterials {
        _id: unwrapped_doc._id.to_string(),
        name: unwrapped_doc.name,
        systems: unwrapped_doc.systems,
    };
    Ok(Some(project_json))
}

pub async fn update_material(
    db: &Database,
    oid: ObjectId,
    input: MaterialWithSysIndicator,
) -> mongodb::error::Result<Option<UpdatedMaterials>> {
    let collection = db.collection::<UpdatedMaterialsDocument>("projects");
    let query = doc! {"_id":oid, "systems.name":input.system};

    let pull_update = doc! {"$pull":{"systems.$.materials":{"partNumber":&input.partNumber}}};

    let push_update = doc! {"$push":{"systems.$.materials":{"brand":&input.brand, "partNumber":&input.partNumber, "qty":&input.qty}}};

    let update_options = FindOneAndUpdateOptions::builder()
        .return_document(ReturnDocument::After)
        .build();

    collection
        .find_one_and_update(query.clone(), pull_update, None)
        .await?;

    let project_doc = collection
        .find_one_and_update(query, push_update, update_options)
        .await?;

    if project_doc.is_none() {
        return Ok(None);
    };

    let unwrapped_doc = project_doc.unwrap();
    let project_json = UpdatedMaterials {
        _id: unwrapped_doc._id.to_string(),
        name: unwrapped_doc.name,
        systems: unwrapped_doc.systems,
    };
    Ok(Some(project_json))
}

pub async fn remove_material(
    db: &Database,
    oid: ObjectId,
    input: MaterialWithSysIndicator,
) -> mongodb::error::Result<Option<UpdatedMaterials>> {
    let collection = db.collection::<UpdatedMaterialsDocument>("projects");
    let query = doc! {"_id":oid, "systems.name":input.system};
    let update = doc! {"$pull":{"systems.$.materials":{"partNumber":&input.partNumber}}};

    let update_options = FindOneAndUpdateOptions::builder()
        .return_document(ReturnDocument::After)
        .build();

    let project_doc = collection
        .find_one_and_update(query, update, update_options)
        .await?;

    if project_doc.is_none() {
        return Ok(None);
    };

    let unwrapped_doc = project_doc.unwrap();
    let project_json = UpdatedMaterials {
        _id: unwrapped_doc._id.to_string(),
        name: unwrapped_doc.name,
        systems: unwrapped_doc.systems,
    };
    Ok(Some(project_json))
}
