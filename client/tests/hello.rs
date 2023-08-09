use core::panic;

use clickhouse_client::{
    binary::{decode::ClickHouseDecoder, encode::ClickHouseEncoder},
    protocol::{
        client::{self},
        server::{self, ServerPacketCode},
    },
};

use tokio::{io::AsyncWriteExt, net::TcpStream};
use tracing::info;
use tracing_test::traced_test;

use miette::Result;

#[tokio::test]
#[traced_test]
async fn test_client_hello() -> Result<()> {
    let hello_packet = client::HelloPacket::default().password("default");
    info!("would send packet: {:?}", hello_packet);

    let mut stream = TcpStream::connect("127.0.0.1:9000").await.unwrap();
    let (reader, writer) = stream.split();

    let mut encoder = ClickHouseEncoder::new(writer);
    hello_packet.encode(&mut encoder).await?;

    let mut decoder = ClickHouseDecoder::new(reader);
    let result_code = decoder.decode_u8().await?;
    info!("result code is: {}", result_code);

    if result_code == ServerPacketCode::Hello as u8 {
        info!("receive hello from server");
        let result_packet = server::HelloPacket::decode(&mut decoder).await?;
        info!("result packet is: {:?}", result_packet);
    } else if result_code == ServerPacketCode::Exception as u8 {
        info!("receive exception from server");
        let result_packet = server::ExceptionPacket::decode_all(&mut decoder).await?;
        info!("result packet is: {:?}", result_packet);
    } else {
        panic!("unknown packet code");
    }

    stream.shutdown().await.unwrap();
    Ok(())
}
