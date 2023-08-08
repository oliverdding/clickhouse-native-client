use std::pin::Pin;
use std::time::Duration;

use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncReadExt};

use crate::binary::MAX_VARINT_LEN64;
use crate::error::ClickHouseClientError;

pub struct ClickHouseDecoder<R> {
    reader: Pin<Box<tokio::io::BufReader<R>>>,
}

impl<R> ClickHouseDecoder<R>
where
    R: AsyncRead,
{
    pub fn new(reader: R) -> Self {
        Self {
            reader: Box::pin(tokio::io::BufReader::new(reader)), // TODO: why should I pin reader here?
        }
    }

    async fn guarantee_size(&mut self, size: usize) -> Result<(), ClickHouseClientError> {
        loop {
            if self.reader.buffer().len() < size {
                let future = self.reader.fill_buf();
                match tokio::time::timeout(Duration::from_millis(100), future).await {
                    Ok(value) => match value {
                        Ok(arr) => {
                            if arr.len() == 0 {
                                return Err(ClickHouseClientError::IoError(
                                    std::io::ErrorKind::UnexpectedEof.into(),
                                ));
                            }
                        }
                        Err(e) => {
                            return Err(ClickHouseClientError::IoError(e));
                        }
                    },
                    Err(_) => {
                        return Err(ClickHouseClientError::ReadTimeout);
                    }
                }
            } else {
                return Ok(());
            }
        }
    }

    pub async fn decode_i32(&mut self) -> Result<i32, ClickHouseClientError> {
        self.guarantee_size(4).await?;
        self.reader
            .read_i32_le()
            .await
            .map_err(|e| ClickHouseClientError::IoError(e))
    }

    pub async fn decode_uvarint(&mut self) -> Result<u64, ClickHouseClientError> {
        let mut x = 0_u64;
        let mut s = 0_u32;
        let mut i = 0_usize;
        loop {
            self.guarantee_size(1).await?;
            let future = self.reader.read_u8();
            let b: u8 = match tokio::time::timeout(Duration::from_millis(100), future).await {
                Ok(value) => value.map_err(|e| ClickHouseClientError::IoError(e))?,
                Err(_) => return Err(ClickHouseClientError::ReadTimeout),
            };

            if b < 0x80 {
                if i == MAX_VARINT_LEN64 || i == (MAX_VARINT_LEN64 - 1) && b > 1 {
                    return Err(ClickHouseClientError::UVarintOverFlow);
                }
                return Ok(x | (u64::from(b) << s));
            }

            x |= u64::from(b & 0x7f) << s;
            s += 7;

            i += 1;
        }
    }

    pub async fn decode_string(&mut self) -> Result<String, ClickHouseClientError> {
        let str_len = self.decode_uvarint().await? as usize;
        self.guarantee_size(str_len).await?;

        let buffer = self.reader.buffer();
        let mut temp = vec![0; str_len];
        temp.copy_from_slice(&buffer[..str_len]);

        let result =
            String::from_utf8(temp).map_err(|e| ClickHouseClientError::FromUtf8Error(e))?;

        self.reader.consume(str_len);
        Ok(result)
    }

    pub async fn decode_bool(&mut self) -> Result<bool, ClickHouseClientError> {
        self.guarantee_size(1).await?;
        match self
            .reader
            .read_u8()
            .await
            .map_err(|e| ClickHouseClientError::IoError(e))?
        {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(ClickHouseClientError::UnknownByte),
        }
    }
}

#[cfg(test)]
mod test {
    use bytes::Buf;

    use crate::binary::decode::ClickHouseDecoder;
    use crate::binary::encode::ClickHouseEncoder;
    use crate::binary::MAX_VARINT_LEN64;

    use miette::Result;

    #[tokio::test]
    async fn test_decode_uvarint() -> Result<()> {
        let mut buf = Vec::with_capacity(10);
        for expected in 0..10240 {
            let encoder = ClickHouseEncoder::new(buf);
            encoder.encode_uvarint(expected).await?;
            encoder.flush().await?;
            encoder.shutdown().await?;

            let mut decoder = ClickHouseDecoder::new(buf.as_slice());
            let actual = decoder.decode_uvarint().await?;

            assert_eq!(actual, expected);
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_decode_continus_uvarint() -> Result<()> {
        const MAX: usize = 10000;
        let mut buf = Vec::with_capacity(10);
        let encoder = ClickHouseEncoder::new(buf);
        for expected in 0..(MAX / MAX_VARINT_LEN64) {
            encoder.encode_uvarint(expected as u64).await?;
        }
        encoder.flush().await?;
        encoder.shutdown().await?;

        let mut decoder = ClickHouseDecoder::new(buf.as_slice());
        for expected in 0..(MAX / MAX_VARINT_LEN64) {
            let actual = decoder.decode_uvarint().await?;
            assert_eq!(actual, expected as u64);
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_read_string() -> Result<()> {
        let mut buf = Vec::with_capacity(1024);

        for expected in vec!["hello world", "rust!", "你好", "❤️‍🔥"] {
            let encoder = ClickHouseEncoder::new(buf);
            encoder.encode_string(expected).await?;
            encoder.flush().await?;
            encoder.shutdown().await?;

            let mut decoder = ClickHouseDecoder::new(buf.as_slice());
            let actual = decoder.decode_string().await?;

            assert_eq!(actual, expected);
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_read_bool() -> Result<()> {
        let mut buf = Vec::with_capacity(1);

        for expected in vec![true, false] {
            let encoder = ClickHouseEncoder::new(buf);
            encoder.encode_bool(expected).await?;
            encoder.flush().await?;
            encoder.shutdown().await?;

            let mut decoder = ClickHouseDecoder::new(buf.as_slice());
            let actual = decoder.decode_bool().await?;

            assert_eq!(actual, expected);
        }
        Ok(())
    }
}
