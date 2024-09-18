use tokio::io::{AsyncRead, AsyncReadExt};

use crate::error::{ClickHouseClientError, Result};
use crate::protocol::{MAX_STRING_SIZE, MAX_VARINT_LEN64};

#[async_trait::async_trait]
pub trait ClickHouseDecoder {
    async fn decode_u8(&mut self) -> Result<u8>;

    async fn decode_bool(&mut self) -> Result<bool>;

    async fn decode_i32(&mut self) -> Result<i32>;

    async fn decode_var_uint(&mut self) -> Result<u64>;

    async fn decode_string(&mut self) -> Result<Vec<u8>>;

    async fn decode_utf8_string(&mut self) -> Result<String>;
}

#[async_trait::async_trait]
impl<R> ClickHouseDecoder for R
where
    R: AsyncRead + Unpin + Send + Sync,
{
    async fn decode_u8(&mut self) -> Result<u8> {
        Ok(self.read_u8().await?)
    }

    async fn decode_bool(&mut self) -> Result<bool> {
        match self.read_u8().await? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(ClickHouseClientError::DecodeError(
                "invalid byte when decoding bool".into(),
            )),
        }
    }

    async fn decode_i32(&mut self) -> Result<i32> {
        Ok(self.read_i32_le().await?)
    }

    async fn decode_var_uint(&mut self) -> Result<u64> {
        let mut result = 0_u64;
        for i in 0..MAX_VARINT_LEN64 {
            let b = self.read_u8().await?;
            if (b & 0x80) == 0 {
                if i == (MAX_VARINT_LEN64 - 1) && b > 1 {
                    return Err(ClickHouseClientError::DecodeError(
                        "overflow when decoding var uint".into(),
                    ));
                }
                return Ok(result | (u64::from(b) << (7 * i)));
            }
            result |= u64::from(b & 0x7F) << (7 * i);
        }
        Err(ClickHouseClientError::DecodeError(
            "overflow when decoding var uint".into(),
        ))
    }

    async fn decode_string(&mut self) -> Result<Vec<u8>> {
        let len = self.decode_var_uint().await?;
        if len as usize > MAX_STRING_SIZE {
            return Err(ClickHouseClientError::DecodeError(
                "size is too long when decoding string".into(),
            ));
        }

        if len == 0 {
            return Ok(Vec::new());
        }

        let mut buf = vec![0_u8; len as usize];
        self.read_exact(&mut buf).await?;

        Ok(buf)
    }

    async fn decode_utf8_string(&mut self) -> Result<String> {
        String::from_utf8(self.decode_string().await?).map_err(|e| e.into())
    }
}

#[cfg(test)]
mod test {
    use anyhow::Result;

    use crate::binary::{ClickHouseDecoder, ClickHouseEncoder};
    use crate::protocol::{MAX_STRING_SIZE, MAX_VARINT_LEN64};

    #[tokio::test]
    async fn test_decode_uvarint() -> Result<()> {
        let mut buf: Vec<u8> = Vec::with_capacity(MAX_STRING_SIZE);
        for expected in 0..10240 {
            let _ = buf.encode_var_uint(expected).await?;

            let mut buffer = buf.as_slice();
            let actual = buffer.decode_var_uint().await?;

            assert_eq!(actual, expected);
            buf.clear();
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_decode_continus_uvarint() -> Result<()> {
        const MAX: usize = 10000;
        let mut buf: Vec<u8> = Vec::with_capacity(MAX_VARINT_LEN64 * MAX);
        for expected in 0..(MAX / MAX_VARINT_LEN64) {
            buf.encode_var_uint(expected as u64).await?;
        }

        let mut buffer = buf.as_slice();
        for expected in 0..(MAX / MAX_VARINT_LEN64) {
            let actual = buffer.decode_var_uint().await?;
            assert_eq!(actual, expected as u64);
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_decode_string() -> Result<()> {
        let mut buf: Vec<u8> = Vec::with_capacity(MAX_STRING_SIZE);
        for expected in ["❤️‍🔥",
            "Hello",
            "你好",
            "こんにちは",
            "안녕하세요",
            "Привет"] {
            let _ = buf.encode_utf8_string(expected).await?;

            let mut buffer = buf.as_slice();
            let actual = buffer.decode_utf8_string().await?;

            assert_eq!(actual, expected);
            buf.clear();
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_decode_bool() -> Result<()> {
        let mut buf: Vec<u8> = Vec::with_capacity(2);
        buf.encode_bool(true).await?;
        buf.encode_bool(false).await?;

        let mut buffer = buf.as_slice();

        let actual = buffer.decode_bool().await?;
        assert!(actual);
        let actual = buffer.decode_bool().await?;
        assert!(!actual);
        Ok(())
    }
}
