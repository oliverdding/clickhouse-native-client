use core::panic;

use bytes::Buf;
use clickhouse_client::protocol::{
    client::{self, ClientPacket, ClientPacketCode},
    server::{self, ServerPacket, ServerPacketCode},
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use tracing::info;
use tracing_test::traced_test;

#[traced_test]
#[test]
fn test_write_packet() {
    use std::fs;
    use std::io::Write;

    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open("/tmp/hello")
        .unwrap();

    let mut buf = bytes::BytesMut::new();
    let hello_packet = client::HelloPacket::default();
    let len = hello_packet.encode(&mut buf).unwrap();
    info!("written hello packet size is: {}", len);
    let binding = buf.freeze();
    let chunk = binding.chunk();
    info!("written hello packet is:\n{:?}", chunk);

    file.write_all(chunk).unwrap();
}

#[tokio::test]
async fn test_client_hello() {
    let hello_packet = client::HelloPacket::default();
    println!("would send packet: {:?}", hello_packet);

    let mut buf = bytes::BytesMut::new();
    let len = hello_packet.encode(&mut buf).unwrap();
    info!("written hello packet size is: {}", len);

    let mut stream = TcpStream::connect("127.0.0.1:9000").await.unwrap();

    stream.write_all_buf(&mut buf).await.unwrap();

    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer).await.unwrap();
    let mut buffer = bytes::Bytes::from(buffer);

    assert_eq!(
        buffer.read_uvarint().unwrap(),
        ServerPacketCode::Hello as u64
    );
    let result_packet = server::HelloPacket::new(Box::new(buffer));
    println!("{:?}", result_packet);

    stream.shutdown().await.unwrap();
}
