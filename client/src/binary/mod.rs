pub mod read;
pub mod write;

// see also: https://pkg.go.dev/encoding/binary#pkg-constants
pub const MAX_VARINT_LEN64: usize = 10;
