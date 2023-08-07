use clickhouse_client::protocol::{
    client::{self, ClientPacket},
    server::{self, ServerPacket},
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use miette::{IntoDiagnostic, Result};

#[tokio::test]
async fn test_client_hello() -> Result<()> {
    let mut hello_packet = client::HelloPacket::new().build();

    println!("{:?}", hello_packet.chunk());

    let mut stream = TcpStream::connect("127.0.0.1:9000")
        .await
        .into_diagnostic()?;

    stream
        .write_all_buf(&mut hello_packet)
        .await
        .into_diagnostic()?;

    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer).await.into_diagnostic()?;
    let buffer = bytes::Bytes::from(buffer);

    let result_packet = server::HelloPacket::new(Box::new(buffer));
    println!("{:?}", result_packet);

    stream.shutdown().await.into_diagnostic()?;
    Ok(())
}
