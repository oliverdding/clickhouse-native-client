use tokio::io::AsyncWrite;

use crate::error::Result;
use crate::protocol::client::{ClickHouseWritePacketCode, ClientPacketCode};

pub trait ClickHouseWritePingPacket: ClickHouseWritePacketCode {
    fn write_ping_packet(
        &mut self,
    ) -> impl std::future::Future<Output = Result<usize>> + Send;
}

impl<R> ClickHouseWritePingPacket for R
where
    R: AsyncWrite + Unpin + Send + Sync,
{
    async fn write_ping_packet(&mut self) -> Result<usize> {
        self.write_packet_code(ClientPacketCode::Ping).await
    }
}
