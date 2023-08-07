pub enum ClientPackets {
    Hello = 0,
    Query = 1,
    Data = 2,
    Cancel = 3,
    Ping = 4,
    TableStatus = 5,
}

pub struct HelloPacket {
    pub client_name: String,
    pub version_major: u64,
    pub version_minor: u64,
    pub protocol_version: u64,
    pub database: String,
    pub username: String,
    pub password: String,
}
