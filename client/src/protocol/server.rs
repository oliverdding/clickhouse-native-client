use crate::binary::{ClickHouseDecoder, ClickHouseDecoderExt};
use crate::error::Result;
use tokio::io::AsyncRead;

#[derive(PartialEq, Copy, Clone)]
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

impl From<u8> for ServerPacketCode {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Hello,
            1 => Self::Data,
            2 => Self::Exception,
            3 => Self::Progress,
            4 => Self::Pong,
            5 => Self::EndOfStream,
            6 => Self::ProfileInfo,
            7 => Self::Totals,
            8 => Self::Extremes,
            9 => Self::TablesStatusResponse,
            10 => Self::Log,
            11 => Self::TableColumns,
            12 => Self::UUIDs,
            13 => Self::ReadTaskRequest,
            14 => Self::ProfileEvents,
            _ => unreachable!(),
        }
    }
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

#[derive(Debug, Clone)]
pub struct ExceptionPacket {
    pub code: i32,
    pub name: String,
    pub message: String,
    pub stack_trace: String,
    pub nested: bool,
}

#[derive(Debug, Clone)]
pub struct PongPacket {}

pub trait ClickHouseRead {
    async fn read_packet_code(&mut self) -> Result<ServerPacketCode>;
    async fn read_hello_packet(&mut self) -> Result<HelloPacket>;
    async fn read_exception_packet(&mut self) -> Result<Vec<ExceptionPacket>>;
}

impl<R> ClickHouseRead for R
where
    R: AsyncRead + Unpin + Send + Sync,
{
    async fn read_packet_code(&mut self) -> Result<ServerPacketCode> {
        Ok(ServerPacketCode::from(self.decode_u8().await?))
    }

    async fn read_hello_packet(&mut self) -> Result<HelloPacket> {
        let name = self.decode_utf8_string().await?;
        let version_major = self.decode_var_uint().await?;
        let version_minor = self.decode_var_uint().await?;
        let revision = self.decode_var_uint().await?;
        let tz = self.decode_utf8_string().await?;
        let display_name = self.decode_utf8_string().await?;
        let version_patch = self.decode_var_uint().await?;
        Ok(HelloPacket {
            name,
            version_major,
            version_minor,
            revision,
            tz,
            display_name,
            version_patch,
        })
    }

    async fn read_exception_packet(&mut self) -> Result<Vec<ExceptionPacket>> {
        let mut exception_list = Vec::new();
        loop {
            let code = self.decode_i32().await?;
            let name = self.decode_utf8_string().await?;
            let message = self.decode_utf8_string().await?;
            let stack_trace = self.decode_utf8_string().await?;
            let nested = self.decode_bool().await?;
            exception_list.push(ExceptionPacket {
                code,
                name: name.to_owned(),
                message: message.to_owned(),
                stack_trace: stack_trace.to_owned(),
                nested: nested,
            });
            if !nested {
                break;
            }
        }
        Ok(exception_list)
    }
}
