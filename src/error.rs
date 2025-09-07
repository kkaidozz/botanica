use std::fmt;

/// Database error types for botanical operations
#[derive(Debug)]
pub enum DatabaseError {
    /// SQLx database error
    SqlxError(sqlx::Error),
    
    /// Migration error
    MigrationError(String),
    
    /// Configuration error
    ConfigError(String),
    
    /// Validation error
    ValidationError(String),
    
    /// Not found error
    NotFound(String),
    
    /// Constraint violation error
    ConstraintViolation(String),
    
    /// ContextLite integration error
    ContextLiteError(String),
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DatabaseError::SqlxError(e) => write!(f, "Database error: {}", e),
            DatabaseError::MigrationError(msg) => write!(f, "Migration error: {}", msg),
            DatabaseError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            DatabaseError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            DatabaseError::NotFound(msg) => write!(f, "Not found: {}", msg),
            DatabaseError::ConstraintViolation(msg) => write!(f, "Constraint violation: {}", msg),
            DatabaseError::ContextLiteError(msg) => write!(f, "ContextLite error: {}", msg),
        }
    }
}

impl std::error::Error for DatabaseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            DatabaseError::SqlxError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<sqlx::Error> for DatabaseError {
    fn from(error: sqlx::Error) -> Self {
        DatabaseError::SqlxError(error)
    }
}

impl DatabaseError {
    /// Create a new migration error
    pub fn migration<S: Into<String>>(msg: S) -> Self {
        DatabaseError::MigrationError(msg.into())
    }
    
    /// Create a new configuration error
    pub fn config<S: Into<String>>(msg: S) -> Self {
        DatabaseError::ConfigError(msg.into())
    }
    
    /// Create a new validation error
    pub fn validation<S: Into<String>>(msg: S) -> Self {
        DatabaseError::ValidationError(msg.into())
    }
    
    /// Create a new not found error
    pub fn not_found<S: Into<String>>(msg: S) -> Self {
        DatabaseError::NotFound(msg.into())
    }
    
    /// Create a new constraint violation error
    pub fn constraint<S: Into<String>>(msg: S) -> Self {
        DatabaseError::ConstraintViolation(msg.into())
    }
    
    /// Create a new ContextLite error
    pub fn contextlite<S: Into<String>>(msg: S) -> Self {
        DatabaseError::ContextLiteError(msg.into())
    }
}