use std::sync::Arc;
use tokio_postgres::{Client, NoTls};

/// Database configuration and initialization module
/// Following Single Responsibility Principle - this module only handles database setup

// Use 127.0.0.1 instead of localhost to ensure TCP connection to Docker container
// localhost might try Unix socket which could connect to local PostgreSQL if running
const DB_CONNECTION_STRING: &str =
    "host=127.0.0.1 user=postgres password=postGr3s1245xSDI dbname=rust_app_db port=5431";

const SCHEMA_INIT_SQL: &str = "CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL
)";

/// Initialize database connection and return the client
/// Spawns a background task to handle the connection
pub async fn init_database() -> Result<Arc<Client>, Box<dyn std::error::Error>> {
    // Establish database connection
    let (client, connection) = tokio_postgres::connect(DB_CONNECTION_STRING, NoTls).await?;

    // Spawn connection handler in background
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Database connection error: {}", e);
        }
    });

    // Initialize database schema
    initialize_schema(&client).await?;

    Ok(Arc::new(client))
}

/// Initialize database schema by creating tables if they don't exist
async fn initialize_schema(client: &Client) -> Result<(), tokio_postgres::Error> {
    client.execute(SCHEMA_INIT_SQL, &[]).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_sql_is_valid() {
        // Verify the schema SQL contains expected elements
        assert!(SCHEMA_INIT_SQL.contains("CREATE TABLE"));
        assert!(SCHEMA_INIT_SQL.contains("users"));
        assert!(SCHEMA_INIT_SQL.contains("email TEXT NOT NULL UNIQUE"));
        assert!(SCHEMA_INIT_SQL.contains("password TEXT NOT NULL"));
    }

    #[test]
    fn test_connection_string_format() {
        // Verify connection string has expected format
        assert!(DB_CONNECTION_STRING.contains("host="));
        assert!(DB_CONNECTION_STRING.contains("user="));
        assert!(DB_CONNECTION_STRING.contains("dbname="));
    }
}
