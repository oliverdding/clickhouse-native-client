use thiserror::Error;
use miette::Diagnostic;

#[derive(Error, Diagnostic, Debug)]
pub enum ClickHouseClientError {
    #[error(transparent)]
    #[diagnostic(code(my_lib::io_error))]
    IoError(#[from] std::io::Error),

    #[error("overflow when reading uvarint")]
    #[diagnostic(code(clickhouse_client::binary::read))]
    UVarintOverFlow,
}
