use mongodb::Database;

#[derive(Clone, Debug)]
pub struct AppState {
    pub db: Database,
}
