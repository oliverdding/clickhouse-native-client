use tokio::io::AsyncWrite;

use crate::binary::ClickHouseEncoder;
use crate::error::Result;
use crate::protocol::client::{ClickHouseWritePacketCode, ClientPacketCode};
use crate::protocol::{
    CLICKHOUSE_CLIENT_NAME, CLICKHOUSE_DEFAULT_DATABASE,
    CLICKHOUSE_DEFAULT_PASSWORD, CLICKHOUSE_DEFAULT_USERNAME,
    CLICKHOUSE_PROTOCOL_VERSION, CLICKHOUSE_VERSION_MAJOR,
    CLICKHOUSE_VERSION_MINOR,
};

#[derive(Debug, Clone)]
pub struct HelloPacket {
    client_name: String,

    version_major: u64, // client major version
    version_minor: u64, // client minor version

    // ProtocolVersion is TCP protocol version of client.
    //
    // Usually it is equal to the latest compatible server revision, but
    // should not be confused with it.
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

pub trait ClickHouseWriteHelloPacket: ClickHouseWritePacketCode {
    fn write_hello_packet(
        &mut self,
        x: HelloPacket,
    ) -> impl std::future::Future<Output = Result<usize>> + Send;
}

impl<R> ClickHouseWriteHelloPacket for R
where
    R: AsyncWrite + Unpin + Send + Sync,
{
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
}

#[cfg(test)]
mod test {
    use anyhow::Result;
    use tracing_test::traced_test;

    use crate::protocol::client;
    use crate::protocol::client::hello::ClickHouseWriteHelloPacket;

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

