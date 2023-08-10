pub const MAX_STRING_SIZE: usize = 1 << 30;
// see also: https://pkg.go.dev/encoding/binary#pkg-constants
pub const MAX_VARINT_LEN64: usize = 10;

pub const CLICKHOUSE_CLIENT_NAME: &str = "clickhouse-native-client";
pub const CLICKHOUSE_VERSION_MAJOR: u64 = 0;
pub const CLICKHOUSE_VERSION_MINOR: u64 = 1;
pub const CLICKHOUSE_PROTOCOL_VERSION: u64 = 54451;

pub const CLICKHOUSE_DEFAULT_DATABASE: &str = "default";
pub const CLICKHOUSE_DEFAULT_USERNAME: &str = "default";
pub const CLICKHOUSE_DEFAULT_PASSWORD: &str = "";