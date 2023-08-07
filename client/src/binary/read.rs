use miette::{IntoDiagnostic, Result};

use crate::error::ClickHouseClientError;

use crate::binary::MAX_VARINT_LEN64;

pub(crate) trait Read {
    fn read_uvarint(&mut self) -> Result<u64>;
    fn read_string(&mut self) -> Result<String>;
    fn read_bool(&mut self) -> Result<bool>;
}

impl<T> Read for T
where
    T: bytes::Buf,
{
    fn read_uvarint(&mut self) -> Result<u64> {
        let mut x = 0_u64;
        let mut s = 0_u32;
        let mut i = 0_usize;
        loop {
            let b: u8 = self.get_u8();

            if b < 0x80 {
                if i == MAX_VARINT_LEN64 || i == (MAX_VARINT_LEN64 - 1) && b > 1 {
                    return Err(ClickHouseClientError::UVarintOverFlow.into());
                }
                return Ok(x | (u64::from(b) << s));
            }

            x |= u64::from(b & 0x7f) << s;
            s += 7;

            i += 1;
        }
    }

    fn read_string(&mut self) -> Result<String> {
        let str_len = self.read_uvarint()? as usize;
        let mut buffer = vec![0_u8; str_len];
        self.copy_to_slice(buffer.as_mut());
        Ok(String::from_utf8(buffer).into_diagnostic()?)
    }

    fn read_bool(&mut self) -> Result<bool> {
        if self.get_u8() == 0_u8 {
            Ok(false)
        } else {
            Ok(true)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::binary::read::Read;
    use crate::binary::write::Write;
    use crate::binary::MAX_VARINT_LEN64;

    #[test]
    fn test_read_uvarint() {
        let mut buf = bytes::BytesMut::with_capacity(10);
        for expected in 0..10240 {
            let _ = buf.write_uvarint(expected);

            let mut buffer = buf.clone().freeze();
            buf.clear();

            let actual = buffer.read_uvarint();

            assert_eq!(actual.unwrap(), expected);
        }
    }

    #[test]
    fn test_write_then_read_uvarint() {
        const MAX: usize = 10000;
        let mut buf = bytes::BytesMut::with_capacity(MAX);
        for expected in 0..(MAX / MAX_VARINT_LEN64) {
            let _ = buf.write_uvarint(expected as u64);
        }
        let mut buffer = buf.freeze();
        for expected in 0..(MAX / MAX_VARINT_LEN64) {
            let actual = buffer.read_uvarint();
            assert_eq!(actual.unwrap(), expected as u64);
        }
    }

    #[test]
    fn test_read_string() {
        let mut buf = bytes::BytesMut::with_capacity(1024);

        for expected in vec!["hello world", "rust!", "你好", "❤️‍🔥"] {
            let _ = buf.write_string(expected);

            let mut buffer = buf.clone().freeze();
            buf.clear();

            let actual = buffer.read_string();

            assert_eq!(actual.unwrap(), expected);
        }
    }

    #[test]
    fn test_read_bool() {
        let mut buf = bytes::BytesMut::with_capacity(1024);

        for expected in vec![true, false] {
            let _ = buf.write_bool(expected);

            let mut buffer = buf.clone().freeze();
            buf.clear();

            let actual = buffer.read_bool();

            assert_eq!(actual.unwrap(), expected);
        }
    }
}
