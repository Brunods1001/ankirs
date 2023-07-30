use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};


pub struct DatabaseService {
    pub pool: SqlitePool,
}

impl DatabaseService {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;
        Ok(Self { pool })
    }
}
