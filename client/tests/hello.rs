use core::panic;

use bytes::Buf;
use clickhouse_client::binary::read::Read;
use clickhouse_client::protocol::{
    client::{self, ClientPacket, ClientPacketCode},
    server::{self, ServerPacket, ServerPacketCode},
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[tokio::test]
async fn test_ping_pong() {
    let mut stream = TcpStream::connect("127.0.0.1:9000").await.unwrap();

    stream.write_u8(ClientPacketCode::Ping as u8).await.unwrap();

    let result = stream.read_u8().await.unwrap();
    println!("ping");

    if ServerPacketCode::Pong as u8 == result {
        println!("pong");
    } else if ServerPacketCode::Exception as u8 == result {
        println!("exception");
        let mut buffer = Vec::new();
        stream.read_to_end(&mut buffer).await.unwrap();
        let mut buffer = bytes::Bytes::from(buffer);
        let result_packet = server::HelloPacket::new(Box::new(buffer));
        println!("{:?}", result_packet);
    } else {
        panic!("unknown packet type: {}", result);
    }

    stream.shutdown().await.unwrap();
}

#[test]
fn test_write_packet() {
    use std::fs;
    use std::io::Write;

    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open("/tmp/hello")
        .unwrap();

    let hello_packet = client::HelloPacket::new();
    let buf = hello_packet.build();
    let chunk = buf.chunk();
    println!("{:?}", chunk);

    file.write_all(chunk).unwrap();
}

#[tokio::test]
async fn test_client_hello() {
    let hello_packet = client::HelloPacket::new();
    println!("would send packet: {:?}", hello_packet);

    let mut buf = hello_packet.build();

    let mut stream = TcpStream::connect("127.0.0.1:9000").await.unwrap();

    stream.write_all_buf(&mut buf).await.unwrap();

    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer).await.unwrap();
    let mut buffer = bytes::Bytes::from(buffer);

    assert_eq!(buffer.read_uvarint().unwrap(), ServerPacketCode::Hello as u64);
    let result_packet = server::HelloPacket::new(Box::new(buffer));
    println!("{:?}", result_packet);

    stream.shutdown().await.unwrap();
}
