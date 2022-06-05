use thiserror::Error;

#[derive(Error, Debug)]
pub enum KvsError {
    #[error("example")]
    ExampleError,

    #[error("serde_json failed, error: {source:?}")]
    SerdeJsonError {
        #[from]
        source: serde_json::Error,
    },

    #[error("io failed, error: {source:?}")]
    IoError {
        #[from]
        source: std::io::Error,
    },

    #[error("Key not found")]
    KeyNotFound,
}
