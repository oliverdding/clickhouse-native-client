use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClickHouseClientError {
    #[error("decode error: {0}")]
    DecodeError(String),

    #[error("encode error: {0}")]
    EncodeError(String),

    #[error("server exception: {code} {name} - {message}\n{stack_trace}")]
    ServerException {
        code: i32,
        name: String,
        message: String,
        stack_trace: String,
    },

    #[error("timeout when reading from remote")]
    ReadTimeout,

    #[error(transparent)]
    Utf8Error(#[from] std::string::FromUtf8Error),

    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

pub type Result<T, E=ClickHouseClientError> = std::result::Result<T,E>;
