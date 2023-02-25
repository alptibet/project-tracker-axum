use mongodb::options::ClientOptions;
use mongodb::{Client, Database};
use std::env;

pub async fn init_db() -> Database {
    let db = match connect().await {
        Ok(_database) => _database,
        Err(_error) => {
            eprintln!("Error connecting to database: {_error:?}");
            panic!();
        }
    };
    db
}

async fn connect() -> mongodb::error::Result<Database> {
    let mongo_uri = env::var("MONGO_URI").expect("MONGO URI MUST BE SET");
    let mongodb_name = env::var("MONGO_DB_NAME").expect("MONGODB NAME MUST BE SET");
    let client_options = ClientOptions::parse(mongo_uri).await?;
    let client = Client::with_options(client_options)?;
    let database = client.database(mongodb_name.as_str());
    println!("Connected to MONGODB!");
    Ok(database)
}
