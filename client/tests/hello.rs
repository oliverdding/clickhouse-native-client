use core::panic;

use bytes::Buf;
use clickhouse_client::{
    binary::decode::ClickHouseDecoder,
    protocol::{
        client::{self, ClientPacket},
        server::{self, ServerPacketCode},
    },
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use tracing::info;
use tracing_test::traced_test;

use miette::Result;

#[tokio::test]
#[traced_test]
async fn test_client_hello() -> Result<()> {
    let hello_packet = client::HelloPacket::default();
    info!("would send packet: {:?}", hello_packet);

    let mut buf = bytes::BytesMut::new();
    let _len = hello_packet.encode(&mut buf)?;

    let mut stream = TcpStream::connect("127.0.0.1:9000").await.unwrap();
    let (reader, mut writer) = stream.split();

    writer.write_all_buf(&mut buf).await.unwrap();
    let mut decoder = ClickHouseDecoder::new(reader);

    let result_code = decoder.decode_uvarint().await?;
    info!("result code is: {}", result_code);

    assert_eq!(result_code, ServerPacketCode::Hello as u64);
    let result_packet = server::HelloPacket::decode(decoder).await?;
    info!("{:?}", result_packet);

    stream.shutdown().await.unwrap();
    Ok(())
}
