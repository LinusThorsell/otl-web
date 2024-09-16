use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Not found: {0}")]
    NotFound(String)
}
