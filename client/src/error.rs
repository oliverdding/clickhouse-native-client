use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum ClickHouseClientError {
    #[error("unknown byte from remote")]
    #[diagnostic(code(clickhouse_client::binary::decode), url(docsrs))]
    UnknownByte,

    #[error("overflow when reading uvarint")]
    #[diagnostic(code(clickhouse_client::binary::decode), url(docsrs))]
    UVarintOverFlow,

    #[error("timeout when reading from remote")]
    #[diagnostic(code(clickhouse_client::binary::decode), url(docsrs))]
    ReadTimeout,

    #[error(transparent)]
    #[diagnostic(code(clickhouse_client::binary::decode))]
    FromUtf8Error(#[from] std::string::FromUtf8Error),

    #[error(transparent)]
    #[diagnostic(code(clickhouse_client::binary::decode))]
    IoError(#[from] std::io::Error),
}
