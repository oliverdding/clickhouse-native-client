use std::pin::Pin;

use tokio::io::{AsyncWrite, AsyncWriteExt};

use crate::error::ClickHouseClientError;

pub struct ClickHouseEncoder<R> {
    writer: Pin<Box<tokio::io::BufWriter<R>>>,
}

impl<R> ClickHouseEncoder<R>
where
    R: AsyncWrite,
{
    pub fn new(writer: R) -> Self {
        Self {
            writer: Box::pin(tokio::io::BufWriter::new(writer)),
        }
    }

    pub async fn encode_u8(&mut self, x: u8) -> Result<usize, ClickHouseClientError> {
        self.writer.write_u8(x).await?;
        Ok(1)
    }

    pub async fn encode_uvarint(&mut self, x: u64) -> Result<usize, ClickHouseClientError> {
        let mut i = 0;
        let mut mx = x;
        while mx >= 0x80 {
            self.writer.write_u8(mx as u8 | 0x80).await?;
            mx >>= 7;
            i += 1;
        }
        self.writer.write_u8(mx as u8).await?;
        Ok(i + 1)
    }

    pub async fn encode_string(&mut self, x: &str) -> Result<usize, ClickHouseClientError> {
        let str_len = x.len();
        let header_len = self.encode_uvarint(str_len as u64).await?;
        self.writer.write_all(x.as_bytes()).await?;
        Ok(header_len + str_len)
    }

    pub async fn encode_bool(&mut self, x: bool) -> Result<usize, ClickHouseClientError> {
        self.writer.write_u8(x as u8).await?;
        Ok(1)
    }

    pub async fn flush(&mut self) -> Result<(), ClickHouseClientError> {
        self.writer.flush().await?;
        Ok(())
    }

    pub async fn shutdown(&mut self) -> Result<(), ClickHouseClientError> {
        self.writer.shutdown().await?;
        Ok(())
    }

    pub fn get_ref(&self) -> &R {
        self.writer.get_ref()
    }
}

#[cfg(test)]
mod test {
    use crate::binary::encode::ClickHouseEncoder;
    use miette::Result;

    #[tokio::test]
    async fn test_write_uvarint_1() -> Result<()> {
        let buf = Vec::with_capacity(10);
        let mut encoder = ClickHouseEncoder::new(buf);
        let length = encoder.encode_uvarint(1).await?;
        encoder.flush().await?;

        assert_eq!(length, 1);
        assert_eq!(*encoder.get_ref(), vec![0x01]);
        Ok(())
    }

    #[tokio::test]
    async fn test_write_uvarint_2() -> Result<()> {
        let buf = Vec::with_capacity(10);
        let mut encoder = ClickHouseEncoder::new(buf);
        let length = encoder.encode_uvarint(2).await?;
        encoder.flush().await?;

        assert_eq!(length, 1);
        assert_eq!(*encoder.get_ref(), vec![0x02]);
        Ok(())
    }

    #[tokio::test]
    async fn test_write_uvarint_127() -> Result<()> {
        let buf = Vec::with_capacity(10);
        let mut encoder = ClickHouseEncoder::new(buf);
        let length = encoder.encode_uvarint(127).await?;
        encoder.flush().await?;

        assert_eq!(length, 1);
        assert_eq!(*encoder.get_ref(), vec![0x7f]);
        Ok(())
    }

    #[tokio::test]
    async fn test_write_uvarint_128() -> Result<()> {
        let buf = Vec::with_capacity(10);
        let mut encoder = ClickHouseEncoder::new(buf);
        let length = encoder.encode_uvarint(128).await?;
        encoder.flush().await?;

        assert_eq!(length, 2);
        assert_eq!(*encoder.get_ref(), vec![0x80, 0x01]);
        Ok(())
    }

    #[tokio::test]
    async fn test_write_uvarint_255() -> Result<()> {
        let buf = Vec::with_capacity(10);
        let mut encoder = ClickHouseEncoder::new(buf);
        let length = encoder.encode_uvarint(255).await?;
        encoder.flush().await?;

        assert_eq!(length, 2);
        assert_eq!(*encoder.get_ref(), vec![0xff, 0x01]);
        Ok(())
    }

    #[tokio::test]
    async fn test_write_uvarint_256() -> Result<()> {
        let buf = Vec::with_capacity(10);
        let mut encoder = ClickHouseEncoder::new(buf);
        let length = encoder.encode_uvarint(256).await?;
        encoder.flush().await?;

        assert_eq!(length, 2);
        assert_eq!(*encoder.get_ref(), vec![0x80, 0x02]);
        Ok(())
    }

    #[tokio::test]
    async fn test_write_uvarint_100500() -> Result<()> {
        let buf = Vec::with_capacity(10);
        let mut encoder = ClickHouseEncoder::new(buf);
        let length = encoder.encode_uvarint(100500).await?;
        encoder.flush().await?;

        assert_eq!(length, 3);
        assert_eq!(*encoder.get_ref(), vec![0x94, 0x91, 0x06]);
        Ok(())
    }

    #[tokio::test]
    async fn test_write_string() -> Result<()> {
        let buf = Vec::with_capacity(1024);
        let mut encoder = ClickHouseEncoder::new(buf);
        let length = encoder.encode_string("Hi").await?;
        encoder.flush().await?;

        assert_eq!(length, 3);
        assert_eq!(*encoder.get_ref(), vec![0x02, 0x48, 0x69]);
        Ok(())
    }

    #[tokio::test]
    async fn test_write_bool_true() -> Result<()> {
        let buf = Vec::with_capacity(1);
        let mut encoder = ClickHouseEncoder::new(buf);
        let length = encoder.encode_bool(true).await?;
        encoder.flush().await?;

        assert_eq!(length, 1);
        assert_eq!(*encoder.get_ref(), vec![0x01]);
        Ok(())
    }

    #[tokio::test]
    async fn test_write_bool_false() -> Result<()> {
        let buf = Vec::with_capacity(1);
        let mut encoder = ClickHouseEncoder::new(buf);
        let length = encoder.encode_bool(false).await?;
        encoder.flush().await?;

        assert_eq!(length, 1);
        assert_eq!(*encoder.get_ref(), vec![0x00]);
        Ok(())
    }
}
