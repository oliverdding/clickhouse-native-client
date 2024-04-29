use tokio::io::{AsyncRead, AsyncReadExt};

use crate::error::{ClickHouseClientError, Result};
use crate::protocol::{MAX_STRING_SIZE, MAX_VARINT_LEN64};

use futures::future::FutureExt;

pub trait ClickHouseDecoder {
    fn decode_u8(
        &mut self,
    ) -> impl std::future::Future<Output = Result<u8>> + Send;

    fn decode_i32(
        &mut self,
    ) -> impl std::future::Future<Output = Result<i32>> + Send;

    fn decode_var_uint(
        &mut self,
    ) -> impl std::future::Future<Output = Result<u64>> + Send;

    fn decode_string(
        &mut self,
    ) -> impl std::future::Future<Output = Result<Vec<u8>>> + Send;
}

pub trait ClickHouseDecoderExt: ClickHouseDecoder {
    fn decode_bool(
        &mut self,
    ) -> impl std::future::Future<Output = Result<bool>> + Send {
        self.decode_u8().map(|x| match x {
            Ok(0) => Ok(false),
            Ok(1) => Ok(true),
            Ok(_) => Err(ClickHouseClientError::DecodeError(
                "invalid byte when decoding bool".into(),
            )),
            Err(e) => Err(e),
        })
    }

    fn decode_utf8_string(
        &mut self,
    ) -> impl std::future::Future<Output = Result<String>> + Send {
        self.decode_string().map(|x| match x {
            Ok(x) => String::from_utf8(x).map_err(|e| e.into()),
            Err(e) => Err(e),
        })
    }
}

impl<T: ?Sized> ClickHouseDecoderExt for T where T: ClickHouseDecoder {}

impl<R> ClickHouseDecoder for R
where
    R: AsyncRead + Unpin + Send + Sync,
{
    async fn decode_u8(&mut self) -> Result<u8> {
        Ok(self.read_u8().await?)
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
}

#[cfg(test)]
mod test {
    use crate::binary::decode::ClickHouseDecoderExt;
    use crate::binary::ClickHouseDecoder;
    use crate::binary::ClickHouseEncoder;
    use crate::protocol::MAX_STRING_SIZE;
    use crate::protocol::MAX_VARINT_LEN64;

    use anyhow::Result;

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
        for expected in
            vec!["❤️‍🔥", "Hello", "你好", "こんにちは", "안녕하세요", "Привет"]
        {
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
        assert_eq!(actual, true);
        let actual = buffer.decode_bool().await?;
        assert_eq!(actual, false);
        Ok(())
    }
}
