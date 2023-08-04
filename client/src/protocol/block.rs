pub enum Modes {
    NONE = 0x02,
    LZ4 = 0x82,
    ZSTD = 0x90,
}

pub struct Block {
    pub checksum: u128,
    pub raw_size: u32,
    pub data_size: u32,
    pub mode: Modes,
    pub compressed_data: Vec<u8>,
}
