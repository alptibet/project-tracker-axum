use futures::StreamExt;
use mongodb::{
    bson::{self, doc},
    Database,
};

use crate::models::{
    projects::{Project, ProjectDocument},
    systems::SysWithScope,
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

    let pipeline = stage_lookup;
    let mut results = collection.aggregate(pipeline, None).await?;

    let mut projects: Vec<Project> = vec![];

    while let Some(result) = results.next().await {
        let doc: ProjectDocument = bson::from_document(result?)?;
        let mut systems: Vec<SysWithScope> = vec![];
        for system in doc.systems {
            let scope = system.get_str("scope").unwrap().to_string();
            let sys_name = system
                .get("system")
                .unwrap()
                .as_document()
                .unwrap()
                .get_str("name")
                .unwrap()
                .to_string();
            systems.push(SysWithScope {
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
            contractor: doc.contractor.get_str("name").unwrap().to_string(),
            systems,
        };
        projects.push(projects_json);
    }
    Ok(projects)
}
