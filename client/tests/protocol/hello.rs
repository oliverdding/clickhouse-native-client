use core::panic;

use clickhouse_client::protocol::{
    client::{self, ClickHouseWriteHelloPacket}, server::{ClickHouseRead, ServerPacketCode}
};

use tokio::{io::AsyncWriteExt, net::TcpStream};
use tracing::info;
use tracing_test::traced_test;

use anyhow::Result;

#[traced_test]
#[tokio::test]
async fn hello() -> Result<()> {
    let hello_packet = client::HelloPacket::default().password("default");
    info!("would send packet: {:?}", hello_packet);

    let mut stream = TcpStream::connect("127.0.0.1:9000").await.unwrap();
    let (mut reader, mut writer) = stream.split();

    writer.write_hello_packet(hello_packet).await?;
    match reader.read_packet_code().await? {
        ServerPacketCode::Hello => {
            let result = reader.read_hello_packet().await?;
            info!("received packet: {:?}", result);
        }
        ServerPacketCode::Exception => {
            let result = reader.read_exception_packet().await?;
            panic!("received exception packet: {:?}", result);
        }
        _ => panic!("unexpected packet code"),
    }

    stream.shutdown().await.unwrap();
    Ok(())
}
