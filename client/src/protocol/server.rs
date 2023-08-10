use tokio::io::AsyncRead;

use crate::{binary::ClickHouseDecoder, error::ClickHouseClientError};

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

// TODO: use async-trait for better consistency

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

// impl HelloPacket {
//     pub async fn decode<R>(
//         decoder: &mut ClickHouseDecoder<R>,
//     ) -> Result<Self, ClickHouseClientError>
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
//         let version_patch = decoder.decode_uvarint().await?;
//         Ok(Self {
//             name,
//             version_major,
//             version_minor,
//             revision,
//             tz,
//             display_name,
//             version_patch,
//         })
//     }
// }

#[derive(Debug, Clone)]
pub struct ExceptionPacket {
    pub code: i32,
    pub name: String,
    pub message: String,
    pub stack_trace: String,
    pub nested: bool,
}

// impl ExceptionPacket {
//     pub async fn decode<R>(
//         decoder: &mut ClickHouseDecoder<R>,
//     ) -> Result<Self, ClickHouseClientError>
//     where
//         R: AsyncRead,
//         Self: Sized,
//     {
//         let code = decoder.decode_i32().await?;
//         let name = decoder.decode_string().await?;
//         let message = decoder.decode_string().await?;
//         let stack_trace = decoder.decode_string().await?;
//         let nested = decoder.decode_bool().await?;
//         Ok(Self {
//             code,
//             name: name.to_owned(),
//             message: message.to_owned(),
//             stack_trace: stack_trace.to_owned(),
//             nested: nested,
//         })
//     }

//     pub async fn decode_all<R>(
//         decoder: &mut ClickHouseDecoder<R>,
//     ) -> Result<Vec<Self>, ClickHouseClientError>
//     where
//         R: AsyncRead,
//         Self: Sized,
//     {
//         let mut exception_list = Vec::new();
//         let mut has_next = true;
//         while has_next {
//             let exception = ExceptionPacket::decode(decoder).await?;
//             has_next = exception.nested;
//             exception_list.push(exception);
//         }
//         Ok(exception_list)
//     }
// }
