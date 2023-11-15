use diesel::ConnectionError;
use diesel::prelude::*;
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use dotenvy::dotenv;

use crate::models::*;

mod models;
mod schema;

async fn create_connection() -> Result<AsyncPgConnection, ConnectionError> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    AsyncPgConnection::establish(&database_url).await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use crate::schema::todos::dsl::*;

    let mut conn = create_connection().await?;

    let results = todos
        // Commenting out the next line will make RustRover use excessive CPU and memory
        .select(Todo::as_select())
        .load(&mut conn)
        .await?;

    println!("{:#?}", results);

    Ok(())
}
