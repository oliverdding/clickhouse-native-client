mod data;
mod hello;
mod ping;
mod query;

pub use data::{BlockInfo, Column, DataPacket};
pub use hello::HelloPacket;
pub use query::{
    ClientInfo, Interface, ClientQueryKind, QueryPacket, Settings, Stage,
};

use tokio::io::AsyncWrite;

use crate::binary::ClickHouseEncoder;

use crate::error::Result;

#[derive(PartialEq, Copy, Clone)]
pub enum ClientPacketCode {
    Hello = 0,       // client part of "handshake"
    Query = 1,       // query start
    Data = 2,        // data block (can be compressed)
    Cancel = 3,      // query cancel
    Ping = 4,        // ping request to server
    TableStatus = 5, // tables status request
}

pub trait ClickHouseWritePacketCode {
    fn write_packet_code(
        &mut self,
        x: ClientPacketCode,
    ) -> impl std::future::Future<Output = Result<usize>> + Send;
}

impl<R> ClickHouseWritePacketCode for R
where
    R: AsyncWrite + Unpin + Send + Sync,
{
    async fn write_packet_code(
        &mut self,
        x: ClientPacketCode,
    ) -> Result<usize> {
        self.encode_u8(x as u8).await
    }
}
