use axum::Json;
use chrono::Utc;
use futures::StreamExt;
use mongodb::{
    bson::{self, doc, oid::ObjectId, Document},
    Database,
};

use crate::models::{
    projects::{Project, ProjectDocument, ProjectInput},
    systems::SysDetails,
};

pub async fn get_all(db: &Database) -> mongodb::error::Result<Vec<Project>> {
    let collection = db.collection::<ProjectDocument>("projects");
    let stage_lookup = vec![
        doc! {"$unwind": {"path":"$systems"}},
        doc! {
            "$lookup":{
                "from":"systems",
                "localField": "systems.system",
                "foreignField":"_id",
                "as": "systems.system"
            },
        },
        doc! {"$unwind": {"path":"$systems.system"}},
        doc! {"$group":
            {
            "_id":"$_id",
            "systems": {
                "$push":"$systems"
            }
        }},
        doc! {
            "$lookup":{
                "from":"projects",
                "localField": "_id",
                "foreignField":"_id",
                "as": "projectDetails"
            },
        },
        doc! {"$unwind": {"path":"$projectDetails"}},
        doc! {"$addFields": {
            "projectDetails.systems":"$systems"
        }},
        doc! {
        "$replaceRoot": {
            "newRoot": "$projectDetails"
            }
        },
        doc! {
        "$lookup":{
            "from":"contractors",
            "localField": "contractor",
            "foreignField":"_id",
            "as": "contractor"
        }},
        doc! {"$unwind": {"path":"$contractor"}},
    ];

    let mut results = collection.aggregate(stage_lookup, None).await?;

    let mut projects: Vec<Project> = vec![];

    while let Some(result) = results.next().await {
        let doc: ProjectDocument = bson::from_document(result?)?;
        let mut systems: Vec<SysDetails> = vec![];
        for system in doc.systems {
            let scope = system.get_str("scope").unwrap().to_string();
            let sys_name = system
                .get("system")
                .unwrap()
                .as_document()
                .unwrap()
                .get_str("name")
                .unwrap_or("No Name")
                .to_string();
            systems.push(SysDetails {
                system: sys_name,
                scope,
            })
        }

        let projects_json = Project {
            _id: doc._id.to_string(),
            name: doc.name,
            address: doc.address,
            active: doc.active,
            completed: doc.completed,
            duration: doc.duration,
            startDate: doc.startDate.to_string(),
            completionDate: doc.completionDate.to_string(),
            contractor: doc
                .contractor
                .get_str("name")
                .unwrap_or("No Name")
                .to_string(),
            systems,
        };
        projects.push(projects_json);
    }
    Ok(projects)
}

pub async fn get_one(db: &Database, oid: ObjectId) -> mongodb::error::Result<Option<Project>> {
    let collection = db.collection::<ProjectDocument>("projects");
    let stage_lookup = vec![
        doc! {"$match": {"_id": oid}},
        doc! {"$unwind": {"path":"$systems"}},
        doc! {
            "$lookup":{
                "from":"systems",
                "localField": "systems.system",
                "foreignField":"_id",
                "as": "systems.system"
            },
        },
        doc! {"$unwind": {"path":"$systems.system"}},
        doc! {"$group":
            {
            "_id":"$_id",
            "systems": {
                "$push":"$systems"
            }
        }},
        doc! {
            "$lookup":{
                "from":"projects",
                "localField": "_id",
                "foreignField":"_id",
                "as": "projectDetails"
            },
        },
        doc! {"$unwind": {"path":"$projectDetails"}},
        doc! {"$addFields": {
            "projectDetails.systems":"$systems"
        }},
        doc! {
        "$replaceRoot": {
            "newRoot": "$projectDetails"
            }
        },
        doc! {
        "$lookup":{
            "from":"contractors",
            "localField": "contractor",
            "foreignField":"_id",
            "as": "contractor"
        }},
        doc! {"$unwind": {"path":"$contractor"}},
    ];

    let mut results = collection.aggregate(stage_lookup, None).await?;

    if let Some(result) = results.next().await {
        let doc: ProjectDocument = bson::from_document(result?)?;
        let mut systems: Vec<SysDetails> = vec![];
        for system in doc.systems {
            let scope = system.get_str("scope").unwrap().to_string();
            let sys_name = system
                .get("system")
                .unwrap()
                .as_document()
                .unwrap()
                .get_str("name")
                .unwrap_or("No Name")
                .to_string();
            systems.push(SysDetails {
                system: sys_name,
                scope,
            })
        }

        let projects_json = Project {
            _id: doc._id.to_string(),
            name: doc.name,
            address: doc.address,
            active: doc.active,
            completed: doc.completed,
            duration: doc.duration,
            startDate: doc.startDate.to_string(),
            completionDate: doc.completionDate.to_string(),
            contractor: doc
                .contractor
                .get_str("name")
                .unwrap_or("No Name")
                .to_string(),
            systems,
        };
        Ok(Some(projects_json))
    } else {
        Ok(None)
    }
}

pub async fn insert_one(
    db: &Database,
    input: Json<ProjectInput>,
) -> mongodb::error::Result<Project> {
    let collection = db.collection::<Document>("projects");
    let chrono_dt: chrono::DateTime<Utc> = input.startDate.parse().unwrap();
    let start_dt: bson::DateTime = chrono_dt.into();
    let chrono_dt: chrono::DateTime<Utc> = input.completionDate.parse().unwrap();
    let completion_dt: bson::DateTime = chrono_dt.into();
    let project_document = doc! {"name": &input.name, "address":&input.address, "active": &input.active, "completed": &input.completed, "startDate": start_dt, "completionDate": completion_dt, "contractor": &input.contractor};
    let insert_one_result = collection.insert_one(project_document, None).await?;
    let project_json = Project {
        _id: insert_one_result
            .inserted_id
            .as_object_id()
            .unwrap()
            .to_string(),
        name: input.name.to_string(),
        address: input.address.to_string(),
        active: input.active,
        completed: input.completed,
        duration: input.duration,
        startDate: input.startDate.to_string(),
        completionDate: input.completionDate.to_string(),
        contractor: input.contractor.to_string(),
        systems: input.systems.to_vec(),
    };
    Ok(project_json)
}
