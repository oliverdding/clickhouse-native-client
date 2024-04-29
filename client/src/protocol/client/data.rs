#[derive(Debug, Clone)]
pub struct DataPacket {
    pub info: BlockInfo,
    pub columns_count: u64,
    pub rows_count: u64,
    pub columns: Vec<Column>,
}

#[derive(Debug, Clone)]
pub struct BlockInfo {
    pub is_overflows: bool,
    pub bucket_num: i32,
}

impl Default for BlockInfo {
    fn default() -> Self {
        Self {
            is_overflows: false,
            bucket_num: -1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Column {
    pub name: String,
    pub column_type: String,
    pub data: Vec<u8>,
}

