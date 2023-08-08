pub trait ClickHouseBufMut {
    fn put_uvarint(&mut self, x: u64) -> usize;
    fn put_string(&mut self, x: &str) -> usize;
    fn put_bool(&mut self, x: bool) -> usize;
}

impl<T> ClickHouseBufMut for T
where
    T: bytes::BufMut,
{
    fn put_uvarint(&mut self, x: u64) -> usize {
        let mut i = 0;
        let mut mx = x;
        while mx >= 0x80 {
            self.put_u8(mx as u8 | 0x80);
            mx >>= 7;
            i += 1;
        }
        self.put_u8(mx as u8);
        i + 1
    }

    fn put_string(&mut self, x: &str) -> usize {
        let str_len = x.len();
        let header_len = self.put_uvarint(str_len as u64);
        self.put_slice(x.as_bytes());
        header_len + str_len
    }

    fn put_bool(&mut self, x: bool) -> usize {
        if x {
            self.put_u8(1);
        } else {
            self.put_u8(0);
        };
        1
    }
}

#[cfg(test)]
mod test {
    use crate::binary::encode::ClickHouseBufMut;

    #[test]
    fn test_write_uvarint_1() {
        let mut buf = bytes::BytesMut::with_capacity(10);

        let length = buf.put_uvarint(1);

        assert_eq!(length, 1);
        assert_eq!(buf, vec![0x01]);
    }

    #[test]
    fn test_write_uvarint_2() {
        let mut buf = bytes::BytesMut::with_capacity(10);

        let length = buf.put_uvarint(2);

        assert_eq!(length, 1);
        assert_eq!(buf, vec![0x02]);
    }

    #[test]
    fn test_write_uvarint_127() {
        let mut buf = bytes::BytesMut::with_capacity(10);

        let length = buf.put_uvarint(127);

        assert_eq!(length, 1);
        assert_eq!(buf, vec![0x7f]);
    }

    #[test]
    fn test_write_uvarint_128() {
        let mut buf = bytes::BytesMut::with_capacity(10);

        let length = buf.put_uvarint(128);

        assert_eq!(length, 2);
        assert_eq!(buf, vec![0x80, 0x01]);
    }

    #[test]
    fn test_write_uvarint_255() {
        let mut buf = bytes::BytesMut::with_capacity(10);

        let length = buf.put_uvarint(255);

        assert_eq!(length, 2);
        assert_eq!(buf, vec![0xff, 0x01]);
    }

    #[test]
    fn test_write_uvarint_256() {
        let mut buf = bytes::BytesMut::with_capacity(10);

        let length = buf.put_uvarint(256);

        assert_eq!(length, 2);
        assert_eq!(buf, vec![0x80, 0x02]);
    }

    #[test]
    fn test_write_uvarint_100500() {
        let mut buf = bytes::BytesMut::with_capacity(10);

        let length = buf.put_uvarint(100500);

        assert_eq!(length, 3);
        assert_eq!(buf, vec![0x94, 0x91, 0x06]);
    }

    #[test]
    fn test_write_string() {
        let mut buf = bytes::BytesMut::with_capacity(1024);

        let length = buf.put_string("Hi");

        assert_eq!(length, 3);
        assert_eq!(buf, vec![0x02, 0x48, 0x69]);
    }

    #[test]
    fn test_write_bool_true() {
        let mut buf = bytes::BytesMut::with_capacity(1);

        let length = buf.put_bool(true);

        assert_eq!(length, 1);
        assert_eq!(buf, vec![0x01]);
    }

    #[test]
    fn test_write_bool_false() {
        let mut buf = bytes::BytesMut::with_capacity(1);

        let length = buf.put_bool(false);

        assert_eq!(length, 1);
        assert_eq!(buf, vec![0x00]);
    }
}