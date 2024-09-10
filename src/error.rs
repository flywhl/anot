use thiserror::Error;

#[derive(Error, Debug)]
pub enum AnnotError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parsing error: {0}")]
    Parse(String),

    #[error("Serialization error: {0}")]
    Serialization(String),
}
