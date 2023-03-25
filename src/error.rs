use std::{io::{self, BufWriter, IntoInnerError}, convert::Infallible};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CustomizedError {
    #[error("error example")]
    ErrorExample,

    #[error("Key not found")]
    KeyNotFound,

    #[error("io error: {}", .source)]
    IoError {
        #[from]
        source: io::Error,
    },

    #[error("serde json error: {}", .source)]
    SerdeJsonError {
        #[from]
        source: serde_json::Error,
    },

    #[error("bufwriter into inner error:{}", .source)]
    BufWriterIntoInnerError {
        #[from]
        source: IntoInnerError<BufWriter<Vec<u8>>>,
    },

    #[error("network error: {}", .source)]
    NetWorkError {
        #[from]
        source: volo_thrift::ResponseError<Infallible>,
    },
}
