use futures::StreamExt;
use mongodb::{
    bson::{self, doc},
    Database,
};

use crate::models::projects::{Project, ProjectDocument};

// pub async fn get_all(db: &Database) -> mongodb::error::Result<Vec<Project>> {
//     let collection = db.collection::<ProjectDocument>("projects");
//     let mut cursor = collection.find(None, None).await?;
//     let mut projects: Vec<Project> = vec![];

//     while let Some(result) = cursor.try_next().await? {
//         let _id = result._id;
//         let name = result.name;
//         let address = result.address;
//         let active = result.active;
//         let completed = result.completed;
//         let duration = result.duration;
//         let startdate = result.startDate;
//         let contractor = result.contractor;
//         // let systems = result.systems;

//         let projects_json = Project {
//             _id: _id.to_string(),
//             name,
//             address,
//             active,
//             completed,
//             duration,
//             startDate: startdate.to_string(),
//             contractor: contractor.try_into(),
//             // systems,
//         };
//         projects.push(projects_json);
//     }
//     Ok(projects)
// }
pub async fn get_all(db: &Database) -> mongodb::error::Result<Vec<Project>> {
    let collection = db.collection::<ProjectDocument>("projects");
    let stage_lookup_contractor = doc! {"$lookup":{
        "from":"contractors",
        "localField": "contractor",
        "foreignField":"_id",
        "as": "contractor"
    }};
    let pipeline = vec![stage_lookup_contractor];
    let mut results = collection.aggregate(pipeline, None).await?;

    let mut projects: Vec<Project> = vec![];

    while let Some(result) = results.next().await {
        let doc: ProjectDocument = bson::from_document(result?)?;

        let projects_json = Project {
            _id: doc._id.to_string(),
            name: doc.name,
            address: doc.address,
            active: doc.active,
            completed: doc.completed,
            duration: doc.duration,
            startDate: doc.startDate.to_string(),
            contractor: doc.contractor[0].get_str("name").unwrap_or("").to_string(),
        };

        projects.push(projects_json);
    }
    Ok(projects)
}
