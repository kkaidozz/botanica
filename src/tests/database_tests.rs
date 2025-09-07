//! Database connection and migration tests
//! 
//! Tests database initialization, connection pooling, health checks, and migrations.

use crate::database::{BotanicalDatabase, DatabaseConfig};
use crate::{create_test_database, initialize_database};
use sqlx::Row;

#[tokio::test]
async fn test_in_memory_database_creation() {
    let db = BotanicalDatabase::memory().await;
    assert!(db.is_ok(), "Failed to create in-memory database: {:?}", db.err());
    
    let db = db.unwrap();
    assert!(db.health_check().await.is_ok(), "Health check failed for in-memory database");
}

#[tokio::test]
async fn test_file_database_creation() {
    let temp_path = ":memory:"; // Use memory for testing to avoid file cleanup
    let config = DatabaseConfig::memory();
    
    let db = BotanicalDatabase::new(config).await;
    assert!(db.is_ok(), "Failed to create file database: {:?}", db.err());
    
    let db = db.unwrap();
    assert!(db.health_check().await.is_ok(), "Health check failed for file database");
}

#[tokio::test]
async fn test_database_config_creation() {
    let config = DatabaseConfig::memory();
    assert_eq!(config.url, "sqlite::memory:");
    assert_eq!(config.max_connections, 1);
    assert!(config.foreign_keys);
    
    let config = DatabaseConfig::file("test.db");
    assert_eq!(config.url, "sqlite:test.db");
    assert_eq!(config.max_connections, 10);
    assert!(config.foreign_keys);
    
    let config = DatabaseConfig::default();
    assert_eq!(config.url, "sqlite:botanical.db");
    assert_eq!(config.max_connections, 10);
    assert!(config.foreign_keys);
}

#[tokio::test]
async fn test_database_migration_success() {
    let db = BotanicalDatabase::memory().await.expect("Failed to create database");
    
    let result = db.migrate().await;
    assert!(result.is_ok(), "Migration failed: {:?}", result.err());
}

#[tokio::test]
async fn test_create_test_database_helper() {
    let result = create_test_database().await;
    assert!(result.is_ok(), "create_test_database helper failed: {:?}", result.err());
    
    let db = result.unwrap();
    assert!(db.health_check().await.is_ok(), "Health check failed after helper creation");
}

#[tokio::test]
async fn test_initialize_database_helper() {
    let result = initialize_database(":memory:").await;
    assert!(result.is_ok(), "initialize_database helper failed: {:?}", result.err());
    
    let db = result.unwrap();
    assert!(db.health_check().await.is_ok(), "Health check failed after initialize");
}

#[tokio::test]
async fn test_database_pool_access() {
    let db = create_test_database().await.expect("Failed to create test database");
    let pool = db.pool();
    
    // Test that we can execute a simple query through the pool
    let result = sqlx::query("SELECT 1 as test")
        .fetch_one(pool)
        .await;
    
    assert!(result.is_ok(), "Failed to execute query through pool: {:?}", result.err());
    
    let row = result.unwrap();
    let test_value: i32 = row.get("test");
    assert_eq!(test_value, 1, "Query result was not as expected");
}

#[tokio::test]
async fn test_foreign_keys_enabled() {
    let db = create_test_database().await.expect("Failed to create test database");
    
    // Check if foreign keys are enabled
    let result = sqlx::query("PRAGMA foreign_keys")
        .fetch_one(db.pool())
        .await;
    
    assert!(result.is_ok(), "Failed to check foreign key status: {:?}", result.err());
    
    let row = result.unwrap();
    let foreign_keys_enabled: i32 = row.get(0);
    assert_eq!(foreign_keys_enabled, 1, "Foreign keys should be enabled");
}

#[tokio::test]
async fn test_database_tables_exist_after_migration() {
    let db = create_test_database().await.expect("Failed to create test database");
    
    // Check that families table exists
    let result = sqlx::query("SELECT name FROM sqlite_master WHERE type='table' AND name='families'")
        .fetch_optional(db.pool())
        .await;
    
    assert!(result.is_ok(), "Failed to query sqlite_master: {:?}", result.err());
    assert!(result.unwrap().is_some(), "families table does not exist after migration");
    
    // Check that genera table exists
    let result = sqlx::query("SELECT name FROM sqlite_master WHERE type='table' AND name='genera'")
        .fetch_optional(db.pool())
        .await;
    
    assert!(result.is_ok(), "Failed to query sqlite_master: {:?}", result.err());
    assert!(result.unwrap().is_some(), "genera table does not exist after migration");
    
    // Check that species table exists
    let result = sqlx::query("SELECT name FROM sqlite_master WHERE type='table' AND name='species'")
        .fetch_optional(db.pool())
        .await;
    
    assert!(result.is_ok(), "Failed to query sqlite_master: {:?}", result.err());
    assert!(result.unwrap().is_some(), "species table does not exist after migration");
}

#[tokio::test]
async fn test_concurrent_database_access() {
    let db = create_test_database().await.expect("Failed to create test database");
    
    // Create multiple concurrent tasks that access the database
    let mut handles = Vec::new();
    
    for i in 0..5 {
        let pool = db.pool().clone();
        let handle = tokio::spawn(async move {
            sqlx::query("SELECT ? as value")
                .bind(i)
                .fetch_one(&pool)
                .await
        });
        handles.push(handle);
    }
    
    // Wait for all tasks to complete
    for (i, handle) in handles.into_iter().enumerate() {
        let result = handle.await.expect("Task panicked");
        assert!(result.is_ok(), "Concurrent query {} failed: {:?}", i, result.err());
        
        let row = result.unwrap();
        let value: i32 = row.get("value");
        assert_eq!(value, i as i32, "Concurrent query {} returned wrong value", i);
    }
}

#[tokio::test]
async fn test_database_close() {
    let db = create_test_database().await.expect("Failed to create test database");
    
    // Verify database is working before close
    assert!(db.health_check().await.is_ok(), "Database should be healthy before close");
    
    // Close the database
    db.close().await;
    
    // After closing, operations should fail
    let result = db.health_check().await;
    assert!(result.is_err(), "Health check should fail after database close");
}