use tokio::io::AsyncRead;

use crate::{binary::decode::ClickHouseDecoder, error::ClickHouseClientError};

#[derive(Copy, Clone)]
pub enum ServerPacketCode {
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

#[derive(Debug, Clone)]
pub struct HelloPacket {
    pub name: String,
    pub version_major: u64,
    pub version_minor: u64,
    pub revision: u64,
    pub tz: String,
    pub display_name: String,
    pub version_patch: u64,
}

impl HelloPacket {
    pub async fn decode<R>(mut decoder: ClickHouseDecoder<R>) -> Result<Self, ClickHouseClientError>
    where
        R: AsyncRead,
        Self: Sized,
    {
        let name = decoder.decode_string().await?;
        let version_major = decoder.decode_uvarint().await?;
        let version_minor = decoder.decode_uvarint().await?;
        let revision = decoder.decode_uvarint().await?;
        let tz = decoder.decode_string().await?;
        let display_name = decoder.decode_string().await?;
        let version_patch = decoder.decode_uvarint().await?;
        Ok(Self {
            name,
            version_major,
            version_minor,
            revision,
            tz,
            display_name,
            version_patch,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ExceptionPacket {
    pub code: i32,
    pub name: String,
    pub message: String,
    pub stack_trace: String,
    pub nested: bool,
}

// impl ExceptionPacket {
//     pub async fn decode<R>(mut decoder: ClickHouseDecoder<R>) -> Result<Self, ClickHouseClientError>
//     where
//         R: AsyncRead,
//         Self: Sized,
//     {
//         let name = decoder.decode_string().await?;
//         let version_major = decoder.decode_uvarint().await?;
//         let version_minor = decoder.decode_uvarint().await?;
//         let revision = decoder.decode_uvarint().await?;
//         let tz = decoder.decode_string().await?;
//         let display_name = decoder.decode_string().await?;
//         let version_patch = decoder.decode_string().await?;
//         Ok(Self {
//             code,
//             name: name.to_owned(),
//             message: message.to_owned(),
//             stack_trace: stack_trace.to_owned(),
//             nested: nested,
//         })
//     }
// }
