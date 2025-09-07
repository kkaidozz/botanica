use sqlx::SqlitePool;
use crate::error::DatabaseError;

/// Configuration for the botanical database connection
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    /// Database connection URL (SQLite file path or :memory:)
    pub url: String,
    
    /// Maximum number of connections in the pool
    pub max_connections: u32,
    
    /// Enable foreign key constraints
    pub foreign_keys: bool,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "sqlite:botanical.db".to_string(),
            max_connections: 10,
            foreign_keys: true,
        }
    }
}

impl DatabaseConfig {
    /// Create a new database configuration for in-memory database
    pub fn memory() -> Self {
        Self {
            url: "sqlite::memory:".to_string(),
            max_connections: 1,
            foreign_keys: true,
        }
    }
    
    /// Create a new database configuration for file-based database
    pub fn file<S: AsRef<str>>(path: S) -> Self {
        Self {
            url: format!("sqlite:{}", path.as_ref()),
            max_connections: 10,
            foreign_keys: true,
        }
    }
}

/// Main database connection pool for botanical operations
#[derive(Debug, Clone)]
pub struct BotanicalDatabase {
    /// SQLite connection pool
    pub pool: SqlitePool,
}

impl BotanicalDatabase {
    /// Create a new database connection from configuration
    pub async fn new(config: DatabaseConfig) -> Result<Self, DatabaseError> {
        let pool = SqlitePool::connect(&config.url).await?;
        
        // Enable foreign key constraints if requested
        if config.foreign_keys {
            sqlx::query("PRAGMA foreign_keys = ON")
                .execute(&pool)
                .await?;
        }
        
        Ok(Self { pool })
    }
    
    /// Create a new in-memory database for testing
    pub async fn memory() -> Result<Self, DatabaseError> {
        Self::new(DatabaseConfig::memory()).await
    }
    
    /// Run database migrations to set up tables
    pub async fn migrate(&self) -> Result<(), DatabaseError> {
        crate::migrations::run_migrations(&self.pool).await
    }
    
    /// Check if the database connection is healthy
    pub async fn health_check(&self) -> Result<(), DatabaseError> {
        sqlx::query("SELECT 1")
            .execute(&self.pool)
            .await?;
        Ok(())
    }
    
    /// Get a reference to the underlying connection pool
    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }
    
    /// Close the database connection pool
    pub async fn close(&self) {
        self.pool.close().await;
    }
}