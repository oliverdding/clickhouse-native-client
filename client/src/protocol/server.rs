pub enum ServerPackets {
    Hello = 0,
    Data = 1,
    Exception = 2,
    Progress = 3,
    Pong = 4,
    EndOfStream = 5,
    ProfileInfo = 6,
    Totals = 7,
    Extremes = 8,
    TablesStatusResponse = 9,
    Log = 10,
    TableColumns = 11,
    UUIDs = 12,
    ReadTaskRequest = 13,
    ProfileEvents = 14,
}

pub struct HelloPacket {
    pub name: &str,
    pub version_major: u64,
    pub version_minor: u64,
    pub revision: u64,
    pub revision: u64,
    pub tz: &str,
    pub display_name: &str,
    pub version_patch: &str,
}
