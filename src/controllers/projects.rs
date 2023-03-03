use futures::StreamExt;
use mongodb::{
    bson::{self, doc},
    Database,
};

use crate::models::projects::{Project, ProjectDocument};

pub async fn get_all(db: &Database) -> mongodb::error::Result<Vec<Project>> {
    let collection = db.collection::<ProjectDocument>("projects");
    let stage_lookup_contractor = vec![
        doc! {
        "$lookup":{
            "from":"contractors",
            "localField": "contractor",
            "foreignField":"_id",
            "as": "contractor"
        }},
        doc! {
            "$lookup":{
                "from":"systems",
                "localField": "systems",
                "foreignField":"_id",
                "as": "systems"
            },
        },
    ];

    let pipeline = stage_lookup_contractor;
    let mut results = collection.aggregate(pipeline, None).await?;

    let mut projects: Vec<Project> = vec![];

    while let Some(result) = results.next().await {
        dbg!(&result);
        let doc: ProjectDocument = bson::from_document(result?)?;
        let mut systems: Vec<String> = vec![];
        for system in doc.systems {
            systems.push(system.get_str("name").unwrap_or("No Name").to_string());
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
            contractor: doc.contractor[0]
                .get_str("name")
                .unwrap_or("No Name")
                .to_string(),
            systems,
        };

        projects.push(projects_json);
    }

    Ok(projects)
}
