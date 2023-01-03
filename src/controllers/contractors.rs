use crate::models::contractors::Contractor;
use crate::models::contractors::ContractorDocument;
use futures::stream::TryStreamExt;

use crate::appstate::AppState;

pub async fn find_contractors(state: AppState) -> mongodb::error::Result<Vec<Contractor>> {
    let collection = state.db.collection::<ContractorDocument>("contractors");
    let mut cursor = collection.find(None, None).await?;
    let mut contractors: Vec<Contractor> = vec![];

    while let Some(result) = cursor.try_next().await? {
        let _id = result._id;
        let name = result.name;
        let contractor_json = Contractor {
            _id: _id.to_string(),
            name,
        };
        contractors.push(contractor_json);
    }
    Ok(contractors)
}
