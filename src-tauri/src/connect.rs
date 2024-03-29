

use tokio_postgres::{NoTls, Error};

pub async fn establish_connection() -> Result<(), Error> {
    dotenvy::dotenv().ok(); // Corrected here

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // From here, you can use `client` to interact with your database
    println!("Successfully connected to the database.");

    Ok(())
}
