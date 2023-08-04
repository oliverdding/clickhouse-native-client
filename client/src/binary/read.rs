use miette::{IntoDiagnostic, Result};

use crate::error::ClickHouseClientError;

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
                if i > 9 || i == 9 && b > 1 {
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
