use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum ClickHouseClientError {
    #[error("overflow when reading uvarint")]
    #[diagnostic(code(clickhouse_client::binary::read), url(docsrs))]
    UVarintOverFlow,
}
