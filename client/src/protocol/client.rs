use tokio::io::AsyncWrite;

use crate::{binary::encode::ClickHouseEncoder, error::ClickHouseClientError};

#[derive(Copy, Clone)]
pub enum ClientPacketCode {
    Hello = 0,
    Query = 1,
    Data = 2,
    Cancel = 3,
    Ping = 4,
    TableStatus = 5,
}

impl ClientPacketCode {
    pub async fn encode<R>(
        &self,
        encoder: &mut ClickHouseEncoder<R>,
    ) -> Result<usize, ClickHouseClientError>
    where
        R: AsyncWrite,
    {
        encoder.encode_u8(*self as u8).await?;
        Ok(1)
    }
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

const CLICKHOUSE_CLIENT_NAME: &str = "clickhouse-native-client";
const CLICKHOUSE_VERSION_MAJOR: u64 = 0;
const CLICKHOUSE_VERSION_MINOR: u64 = 1;
const CLICKHOUSE_PROTOCOL_VERSION: u64 = 54451;
const CLICKHOUSE_DATABASE: &str = "default";
const CLICKHOUSE_USERNAME: &str = "default";
const CLICKHOUSE_PASSWORD: &str = "";

impl HelloPacket {
    pub fn default() -> Self {
        Self {
            client_name: CLICKHOUSE_CLIENT_NAME.to_owned(),
            version_major: CLICKHOUSE_VERSION_MAJOR,
            version_minor: CLICKHOUSE_VERSION_MINOR,
            protocol_version: CLICKHOUSE_PROTOCOL_VERSION,
            database: CLICKHOUSE_DATABASE.to_owned(),
            username: CLICKHOUSE_USERNAME.to_owned(),
            password: CLICKHOUSE_PASSWORD.to_owned(),
        }
    }

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

impl HelloPacket {
    pub async fn encode<R>(
        &self,
        encoder: &mut ClickHouseEncoder<R>,
    ) -> Result<usize, ClickHouseClientError>
    where
        R: AsyncWrite,
    {
        let mut len: usize = 0;
        len += ClientPacketCode::Hello.encode(encoder).await?;
        len += encoder.encode_string(&self.client_name).await?;
        len += encoder.encode_uvarint(self.version_major).await?;
        len += encoder.encode_uvarint(self.version_minor).await?;
        len += encoder.encode_uvarint(self.protocol_version).await?;
        len += encoder.encode_string(&self.database).await?;
        len += encoder.encode_string(&self.username).await?;
        len += encoder.encode_string(&self.password).await?;

        encoder.flush().await?;

        Ok(len)
    }
}

#[cfg(test)]
mod test {
    use crate::{binary::encode::ClickHouseEncoder, protocol::client};
    use miette::Result;
    use tracing::info;
    use tracing_test::traced_test;

    #[traced_test]
    #[tokio::test]
    async fn test_client_hello() -> Result<()> {
        let buf = Vec::new();
        let mut encoder = ClickHouseEncoder::new(buf);

        let len = client::HelloPacket::default().encode(&mut encoder).await?;

        info!("written hello packet size is: {}", len);
        info!(
            "written hello packet is:\n{:?}",
            encoder.get_ref().as_slice()
        );
        Ok(())
    }
}
