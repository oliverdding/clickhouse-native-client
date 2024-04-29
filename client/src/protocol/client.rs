use tokio::io::AsyncWrite;

use crate::binary::ClickHouseEncoder;

use crate::error::Result;

use crate::protocol::{
    CLICKHOUSE_CLIENT_NAME, CLICKHOUSE_DEFAULT_DATABASE,
    CLICKHOUSE_DEFAULT_PASSWORD, CLICKHOUSE_DEFAULT_USERNAME,
    CLICKHOUSE_PROTOCOL_VERSION, CLICKHOUSE_VERSION_MAJOR,
    CLICKHOUSE_VERSION_MINOR,
};

#[derive(PartialEq, Copy, Clone)]
pub enum ClientPacketCode {
    Hello = 0,
    Query = 1,
    Data = 2,
    Cancel = 3,
    Ping = 4,
    TableStatus = 5, // TODO: not implemented yet
}

#[derive(Debug, Clone)]
pub struct HelloPacket {
    client_name: String,
    version_major: u64,
    version_minor: u64,
    protocol_version: u64,
    pub database: String,
    pub username: String,
    pub password: String,
}

impl Default for HelloPacket {
    fn default() -> Self {
        Self {
            client_name: CLICKHOUSE_CLIENT_NAME.to_owned(),
            version_major: CLICKHOUSE_VERSION_MAJOR,
            version_minor: CLICKHOUSE_VERSION_MINOR,
            protocol_version: CLICKHOUSE_PROTOCOL_VERSION,
            database: CLICKHOUSE_DEFAULT_DATABASE.to_owned(),
            username: CLICKHOUSE_DEFAULT_USERNAME.to_owned(),
            password: CLICKHOUSE_DEFAULT_PASSWORD.to_owned(),
        }
    }
}

impl HelloPacket {
    pub fn database(mut self, database: impl Into<String>) -> HelloPacket {
        self.database = database.into();
        self
    }

    pub fn username(mut self, username: impl Into<String>) -> HelloPacket {
        self.username = username.into();
        self
    }

    pub fn password(mut self, password: impl Into<String>) -> HelloPacket {
        self.password = password.into();
        self
    }
}

#[derive(Debug, Clone)]
pub struct QueryPacket {
    pub query_id: String,
    pub client_info: ClientInfo,
    pub settings: Settings,
    pub secret: String,
    pub stage: Stage,
    pub compression: u64,
    pub body: String,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Stage {
    FetchColumns = 0,
    WithMergeableState = 1,
    Complete = 2,
}

#[derive(Debug, Clone)]
pub struct ClientInfo {
    pub query_kind: QueryKind,
    pub initial_user: String,
    pub initial_query_id: String,
    pub initial_address: String,
    pub initial_time: i64,
    pub interface: ClientInterface,
    pub os_user: String,
    pub client_hostname: String,
    pub client_name: String,
    pub version_major: u64,
    pub version_minor: u64,
    pub version_patch: u64,
    pub protocol_version: u64,
    pub quota_key: String,
    pub distributed_depth: u64,
    pub otel: bool,
    pub trace_id: String,
    pub span_id: String,
    pub trace_state: String,
    pub trace_flags: u8,
}

#[derive(PartialEq, Debug, Clone)]
pub enum QueryKind {
    None = 0,
    Initial = 1,
    Secondary = 2,
}

#[derive(PartialEq, Debug, Clone)]
pub enum ClientInterface {
    TCP = 1,
    HTTP = 2,
}

#[derive(Debug, Clone)]
pub struct Settings {
    pub key: String,
    pub value: String,
    pub important: bool,
}

#[derive(Debug, Clone)]
pub struct DataPacket {
    pub info: BlockInfo,
    pub columns_count: u64,
    pub rows_count: u64,
    pub columns: Vec<Column>,
}

#[derive(Debug, Clone)]
pub struct BlockInfo {
    pub overflows: bool,
    pub bucket_num: i32,
}

#[derive(Debug, Clone)]
pub struct Column {
    pub name: String,
    pub column_type: String,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct CancelPacket {}

pub trait ClickHouseWrite {
    fn write_packet_code(
        &mut self,
        x: ClientPacketCode,
    ) -> impl std::future::Future<Output = Result<usize>> + Send;
    fn write_hello_packet(
        &mut self,
        x: HelloPacket,
    ) -> impl std::future::Future<Output = Result<usize>> + Send;
    fn write_ping_packet(
        &mut self,
    ) -> impl std::future::Future<Output = Result<usize>> + Send;
}

impl<R> ClickHouseWrite for R
where
    R: AsyncWrite + Unpin + Send + Sync,
{
    async fn write_packet_code(
        &mut self,
        x: ClientPacketCode,
    ) -> Result<usize> {
        self.encode_u8(x as u8).await
    }

    async fn write_hello_packet(&mut self, x: HelloPacket) -> Result<usize> {
        let mut len: usize = 0;
        len += self.write_packet_code(ClientPacketCode::Hello).await?;
        len += self.encode_utf8_string(x.client_name).await?;
        len += self.encode_var_uint(x.version_major).await?;
        len += self.encode_var_uint(x.version_minor).await?;
        len += self.encode_var_uint(x.protocol_version).await?;
        len += self.encode_utf8_string(x.database).await?;
        len += self.encode_utf8_string(x.username).await?;
        len += self.encode_utf8_string(x.password).await?;

        Ok(len)
    }

    async fn write_ping_packet(&mut self) -> Result<usize> {
        self.write_packet_code(ClientPacketCode::Ping).await
    }
}

#[cfg(test)]
mod test {
    use crate::protocol::client::{self, ClickHouseWrite};
    use anyhow::Result;
    use tracing_test::traced_test;

    #[traced_test]
    #[tokio::test]
    async fn test_default_client_hello() -> Result<()> {
        let mut buf: Vec<u8> = Vec::new();

        let len = buf
            .write_hello_packet(client::HelloPacket::default())
            .await?;

        let hello_packet: [u8; 48] = [
            0, 24, 99, 108, 105, 99, 107, 104, 111, 117, 115, 101, 45, 110, 97,
            116, 105, 118, 101, 45, 99, 108, 105, 101, 110, 116, 0, 1, 179,
            169, 3, 7, 100, 101, 102, 97, 117, 108, 116, 7, 100, 101, 102, 97,
            117, 108, 116, 0,
        ];

        assert!(len == 48, "written hello packet size is: {}", len);
        assert!(vec_compare(&buf, &hello_packet));
        Ok(())
    }

    fn vec_compare(va: &[u8], vb: &[u8]) -> bool {
        (va.len() == vb.len()) &&  // zip stops at the shortest
         va.iter()
           .zip(vb)
           .all(|(a,b)| *a == *b)
    }
}
