use tokio::io::{AsyncWrite, AsyncWriteExt};

use crate::error::Result;

pub trait ClickHouseEncoder {
    fn encode_u8(
        &mut self,
        x: u8,
    ) -> impl std::future::Future<Output = Result<usize>> + Send;

    fn encode_bool(
        &mut self,
        x: bool,
    ) -> impl std::future::Future<Output = Result<usize>> + Send;

    fn encode_i32(
        &mut self,
        x: i32,
    ) -> impl std::future::Future<Output = Result<usize>> + Send;

    fn encode_var_uint(
        &mut self,
        x: u64,
    ) -> impl std::future::Future<Output = Result<usize>> + Send;

    fn encode_string(
        &mut self,
        x: impl AsRef<[u8]> + Send,
    ) -> impl std::future::Future<Output = Result<usize>> + Send;

    fn encode_utf8_string(
        &mut self,
        x: impl AsRef<str> + Send,
    ) -> impl std::future::Future<Output = Result<usize>> + Send;
}

impl<R> ClickHouseEncoder for R
where
    R: AsyncWrite + Unpin + Send + Sync,
{
    async fn encode_u8(&mut self, x: u8) -> Result<usize> {
        self.write_u8(x).await?;
        Ok(1)
    }

    async fn encode_bool(&mut self, x: bool) -> Result<usize> {
        self.encode_u8(x as u8).await
    }

    async fn encode_i32(&mut self, x: i32) -> Result<usize> {
        self.write_i32_le(x).await?;
        Ok(4)
    }

    async fn encode_var_uint(&mut self, x: u64) -> Result<usize> {
        let mut i = 0;
        let mut x = x;
        while x >= 0x80 {
            self.write_u8(x as u8 | 0x80).await?;
            x >>= 7;
            i += 1;
        }
        self.write_u8(x as u8).await?;
        Ok(i + 1)
    }

    async fn encode_string(
        &mut self,
        x: impl AsRef<[u8]> + Send,
    ) -> Result<usize> {
        let x = x.as_ref();
        let str_len = x.len();
        let header_len = self.encode_var_uint(str_len as u64).await?;
        self.write_all(x).await?;
        Ok(header_len + str_len)
    }

    async fn encode_utf8_string(
        &mut self,
        x: impl AsRef<str> + Send,
    ) -> Result<usize> {
        self.encode_string(x.as_ref().as_bytes()).await
    }
}

#[cfg(test)]
mod test {
    use crate::{binary::ClickHouseEncoder, protocol::MAX_VARINT_LEN64};
    use anyhow::Result;

    #[tokio::test]
    async fn test_write_var_uint_1() -> Result<()> {
        let mut buf = Vec::with_capacity(MAX_VARINT_LEN64);
        let len = buf.encode_var_uint(1).await?;

        assert_eq!(len, 1);
        assert_eq!(buf, vec![0x01]);
        Ok(())
    }

    #[tokio::test]
    async fn test_write_var_uint_2() -> Result<()> {
        let mut buf = Vec::with_capacity(MAX_VARINT_LEN64);
        let len = buf.encode_var_uint(2).await?;

        assert_eq!(len, 1);
        assert_eq!(buf, vec![0x02]);
        Ok(())
    }

    #[tokio::test]
    async fn test_write_var_uint_127() -> Result<()> {
        let mut buf = Vec::with_capacity(MAX_VARINT_LEN64);
        let len = buf.encode_var_uint(127).await?;

        assert_eq!(len, 1);
        assert_eq!(buf, vec![0x7f]);
        Ok(())
    }

    #[tokio::test]
    async fn test_write_var_uint_128() -> Result<()> {
        let mut buf = Vec::with_capacity(MAX_VARINT_LEN64);
        let len = buf.encode_var_uint(128).await?;

        assert_eq!(len, 2);
        assert_eq!(buf, vec![0x80, 0x01]);
        Ok(())
    }

    #[tokio::test]
    async fn test_write_var_uint_255() -> Result<()> {
        let mut buf = Vec::with_capacity(MAX_VARINT_LEN64);
        let len = buf.encode_var_uint(255).await?;

        assert_eq!(len, 2);
        assert_eq!(buf, vec![0xff, 0x01]);
        Ok(())
    }

    #[tokio::test]
    async fn test_write_var_uint_256() -> Result<()> {
        let mut buf = Vec::with_capacity(MAX_VARINT_LEN64);
        let len = buf.encode_var_uint(256).await?;

        assert_eq!(len, 2);
        assert_eq!(buf, vec![0x80, 0x02]);
        Ok(())
    }

    #[tokio::test]
    async fn test_write_var_uint_100500() -> Result<()> {
        let mut buf = Vec::with_capacity(MAX_VARINT_LEN64);
        let len = buf.encode_var_uint(100500).await?;

        assert_eq!(len, 3);
        assert_eq!(buf, vec![0x94, 0x91, 0x06]);
        Ok(())
    }

    #[tokio::test]
    async fn test_write_string() -> Result<()> {
        let mut buf = Vec::with_capacity(1024);
        let len = buf.encode_utf8_string("Hi").await?;

        assert_eq!(len, 3);
        assert_eq!(buf, vec![0x02, 0x48, 0x69]);
        Ok(())
    }

    #[tokio::test]
    async fn test_write_bool() -> Result<()> {
        let mut buf = Vec::with_capacity(2);
        let len = buf.encode_bool(true).await? + buf.encode_bool(false).await?;

        assert_eq!(len, 2);
        assert_eq!(buf, vec![0x01, 0x00]);
        Ok(())
    }
}
