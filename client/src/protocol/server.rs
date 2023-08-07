use crate::binary::read::Read;
use miette::Result;

pub trait ServerPacket {
    fn new(buf: Box<dyn bytes::Buf>) -> Result<Self> where Self: Sized;
}

pub enum ServerPackets {
    Hello = 0,
    Data = 1,
    Exception = 2,
    Progress = 3,
    Pong = 4,
    EndOfStream = 5,
    ProfileInfo = 6,
    Totals = 7,
    Extremes = 8,
    TablesStatusResponse = 9,
    Log = 10,
    TableColumns = 11,
    UUIDs = 12,
    ReadTaskRequest = 13,
    ProfileEvents = 14,
}

#[derive(Debug, Clone)]
pub struct HelloPacket {
    pub name: String,
    pub version_major: u64,
    pub version_minor: u64,
    pub revision: u64,
    pub tz: String,
    pub display_name: String,
    pub version_patch: String,
}

impl ServerPacket for HelloPacket {
    fn new(mut buf: Box<dyn bytes::Buf>) -> Result<Self> where Self: Sized {
        let name = buf.read_string()?;
        let version_major = buf.read_uvarint()?;
        let version_minor = buf.read_uvarint()?;
        let revision = buf.read_uvarint()?;
        let tz = buf.read_string()?;
        let display_name = buf.read_string()?;
        let version_patch = buf.read_string()?;
        Ok(Self {
            name,
            version_major,
            version_minor,
            revision,
            tz,
            display_name,
            version_patch,
        })
    }
}
