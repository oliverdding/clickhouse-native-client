pub(crate) trait Write {
    fn write_uvarint(&mut self, u64) -> Result<u64>;
    fn write_string(&mut self) -> Result<String>;
    fn write_bool(&mut self) -> Result<bool>;
}
