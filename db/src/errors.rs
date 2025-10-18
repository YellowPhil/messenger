use scylla::errors::{NewSessionError, PrepareError};

#[derive(Debug, thiserror::Error)]
pub enum DbError {
    #[error("Invalid URI: {0}")]
    InvalidUri(#[from] NewSessionError),
    #[error("Prepared statement error: {0}")]
    PreparedStatementError(#[from] PrepareError),
    #[error("Connection error: {0}")]
    ConnectionError(String),
    #[error("Query error: {0}")]
    QueryError(String),
    #[error("Execution error: {0}")]
    ExecutionError(#[from] scylla::errors::ExecutionError),
}
