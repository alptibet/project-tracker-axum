use futures::StreamExt;
use mongodb::{
    bson::{self, doc},
    Database,
};

use crate::models::{
    projects::{Project, ProjectDocument},
    systems::{SysWithScope, SysWithScope2, System},
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
        dbg!(&result);
        let doc: ProjectDocument = bson::from_document(result?)?;
        dbg!(&doc);
        let mut systems: Vec<SysWithScope2> = vec![];
        for system in doc.systems {
            dbg!(system);
            // systems.push(SysWithScope{
            //     system: System {
            //         _id: system._id,
            //         name: system.name
            //     },
            //     scope: system.scope.to_string()
            // });
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
            contractor: doc.contractor.to_string(),
            systems: vec!["DENEME".to_string()],
        };

        projects.push(projects_json);
    }

    Ok(projects)
}
