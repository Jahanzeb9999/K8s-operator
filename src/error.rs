use thiserror::Error;


#[derive(Error, Debug)]
pub enum  Error {
    #[error("Kubernetes Api Error: {0}")]
    KubeError(#[from] kube::Error),

    #[error ("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Operation failed: {0}")]
    OperationFailed(String),

    /// An error indicating that something is missing or invalid
    #[error("Invalid input: {0}")]
    InvalidInput(String),  // Handle invalid input scenarios
    
}

pub type Result<T> = std::result::Result<T, Error>;