use crate::{binary::encode::BatchBufMut, error::ClickHouseClientError};

pub trait ClientPacket {
    fn encode(&self, buf: &mut dyn bytes::BufMut) -> Result<usize, ClickHouseClientError>;
}

#[derive(Copy, Clone)]
pub enum ClientPacketCode {
    Hello = 0,
    Query = 1,
    Data = 2,
    Cancel = 3,
    Ping = 4,
    TableStatus = 5,
}

impl ClientPacket for ClientPacketCode {
    fn encode(&self, buf: &mut dyn bytes::BufMut) -> Result<usize, ClickHouseClientError> {
        buf.put_u8(*self as u8);
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

    pub fn database(mut self, database: &str) -> HelloPacket {
        self.database = database.to_owned();
        self
    }

    pub fn username(mut self, username: &str) -> HelloPacket {
        self.username = username.to_owned();
        self
    }

    pub fn password(mut self, password: &str) -> HelloPacket {
        self.password = password.to_owned();
        self
    }
}

impl ClientPacket for HelloPacket {
    fn encode(&self, mut buf: &mut dyn bytes::BufMut) -> Result<usize, ClickHouseClientError> {
        let mut len: usize = 0;
        len += ClientPacketCode::Hello.encode(buf).unwrap();
        len += buf.put_string(&self.client_name);
        len += buf.put_uvarint(self.version_major);
        len += buf.put_uvarint(self.version_minor);
        len += buf.put_uvarint(self.protocol_version);
        len += buf.put_string(&self.database);
        len += buf.put_string(&self.username);
        len += buf.put_string(&self.password);
        Ok(len)
    }
}

#[cfg(test)]
mod test {
    use crate::protocol::client::{self, ClientPacket};
    use bytes::Buf;
    use tracing::info;
    use tracing_test::traced_test;

    #[traced_test]
    #[test]
    fn test_client_hello() {
        let mut buf = bytes::BytesMut::new();

        let len = client::HelloPacket::default().encode(&mut buf);

        info!("written hello packet size is: {}", len.unwrap());
        info!("written hello packet is:\n{:?}", buf.freeze().chunk());
    }
}
