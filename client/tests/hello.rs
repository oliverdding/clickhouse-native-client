use core::panic;

use bytes::{Buf, BufMut};
use clickhouse_client::protocol::{
    client::{self, ClientPacket, ClientPackets},
    server::{self, ServerPacket, ServerPackets},
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[tokio::test]
async fn test_ping_pong() {
    let mut stream = TcpStream::connect("127.0.0.1:9000").await.unwrap();

    stream.write_u8(ClientPackets::Ping as u8).await.unwrap();

    let result = stream.read_u8().await.unwrap();
    println!("ping");
    println!("{}", result);

    if ServerPackets::Pong as u8 == result {
        println!("pong");
    } else if ServerPackets::Exception as u8 == result {
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

    assert_eq!(buffer.get_u8(), ServerPackets::Hello as u8);
    let result_packet = server::HelloPacket::new(Box::new(buffer));
    println!("{:?}", result_packet);

    stream.shutdown().await.unwrap();
}
