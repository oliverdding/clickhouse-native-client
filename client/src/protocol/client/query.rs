#[derive(Debug, Clone)]
pub struct QueryPacket {
    pub query_id: String,
    pub client_info: ClientInfo,
    pub settings: Vec<Settings>,
    pub secret: String,
    pub stage: Stage,
    pub compression: u64,
    pub body: String,
}

#[derive(Debug, Clone)]
pub struct ClientInfo {
    pub protocol_version: u64,

    pub version_major: u64,
    pub version_minor: u64,
    pub version_patch: u64,

    pub interface: Interface,
    pub query_kind: ClientQueryKind,

    pub initial_user: String,
    pub initial_query_id: String,
    pub initial_address: String,
    pub initial_time: i64,
    
    pub os_user: String,
    pub client_hostname: String,
    pub client_name: String,

    pub quota_key: String,
    pub distributed_depth: u64,
    
    pub otel: bool,
    pub trace_id: String,
    pub span_id: String,
    pub trace_state: String,
    pub trace_flags: u8,
}

#[derive(Debug, Clone)]
pub struct Settings {
    pub key: String,
    pub value: String,
    pub important: bool,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Stage {
    FetchColumns = 0,
    WithMergeableState = 1,
    Complete = 2,
}

#[derive(PartialEq, Debug, Clone)]
pub enum ClientQueryKind {
    None = 0,
    Initial = 1,
    Secondary = 2,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Interface {
    TCP = 1,
    HTTP = 2,
}
