use crate::TryFromBytes;
use byteorder::ReadBytesExt;

#[derive(Debug)]
pub struct HeartBeatResponse {
    pub value: u16,
}

impl TryFromBytes for HeartBeatResponse {
    fn try_from_bytes(bytes: &[u8]) -> anyhow::Result<Self> {
        let mut cursor = std::io::Cursor::new(bytes);
        let value = cursor.read_u16::<byteorder::BigEndian>()?;
        Ok(HeartBeatResponse { value })
    }
}
