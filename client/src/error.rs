use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum ClickHouseClientError {
    #[error("decode error: {0}")]
    #[diagnostic(code(clickhouse_client::binary::decode), url(docsrs))]
    DecodeError(String),

    #[error("encode error: {0}")]
    #[diagnostic(code(clickhouse_client::binary::encode), url(docsrs))]
    EncodeError(String),

    #[error("server exception: {code} {name} - {message}\n{stack_trace}")]
    #[diagnostic(code(clickhouse_client::binary::decode), url(docsrs))]
    ServerException {
        code: i32,
        name: String,
        message: String,
        stack_trace: String,
    },

    #[error("timeout when reading from remote")]
    #[diagnostic(code(clickhouse_client::binary::decode), url(docsrs))]
    ReadTimeout,

    #[error(transparent)]
    #[diagnostic(code(clickhouse_client::binary::decode))]
    Utf8Error(#[from] std::string::FromUtf8Error),

    #[error(transparent)]
    #[diagnostic(code(clickhouse_client::binary::decode))]
    IoError(#[from] std::io::Error),
}

pub type Result<T, E=ClickHouseClientError> = std::result::Result<T,E>;
