use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum ClickHouseClientError {
    #[error("overflow when reading uvarint")]
    #[diagnostic(code(clickhouse_client::binary::decode), url(docsrs))]
    UVarintOverFlow,

    #[error("timeout when reading from socket")]
    #[diagnostic(code(clickhouse_client::binary::decode), url(docsrs))]
    ReadTimeout,
}
