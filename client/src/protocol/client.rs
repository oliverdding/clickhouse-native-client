use crate::binary::write::Write;

pub trait ClientPacket {
    fn new() -> Self;
    fn build(&self) -> Box<dyn bytes::Buf>;
}

pub enum ClientPackets {
    Hello = 0,
    Query = 1,
    Data = 2,
    Cancel = 3,
    Ping = 4,
    TableStatus = 5,
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
    pub fn database(&mut self, database: &str) {
        self.database = database.to_owned();
    }

    pub fn username(&mut self, username: &str) {
        self.username = username.to_owned();
    }

    pub fn password(&mut self, password: &str) {
        self.password = password.to_owned();
    }
}

impl ClientPacket for HelloPacket {
    fn new() -> Self {
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

    fn build(&self) -> Box<dyn bytes::Buf> {
        let mut buf = bytes::BytesMut::new();
        buf.write_string(&self.client_name);
        buf.write_uvarint(self.version_major);
        buf.write_uvarint(self.version_minor);
        buf.write_uvarint(self.protocol_version);
        buf.write_string(&self.database);
        buf.write_string(&self.username);
        buf.write_string(&self.password);
        Box::new(buf.freeze()) // FIXME: temporary value borrowed
    }
}

#[cfg(test)]
mod test {
    use crate::protocol::client::{self, ClientPacket};

    #[test]
    fn test_client_hello() {
        let hello_packet = client::HelloPacket::new().build();

        println!("{:?}", hello_packet.chunk());
    }
}
