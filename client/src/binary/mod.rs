pub mod decode;
pub mod encode;

// see also: https://pkg.go.dev/encoding/binary#pkg-constants
pub const MAX_VARINT_LEN64: usize = 10;
