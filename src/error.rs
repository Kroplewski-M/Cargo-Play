use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("terminal error: {0}")]
    Terminal(#[from] std::io::Error),
    #[error("database error: {0}")]
    Database(#[from] rusqlite::Error),
}

pub type Result<T> = std::result::Result<T, AppError>;
