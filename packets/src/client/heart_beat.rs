use crate::ToBytes;

use super::Codes;

#[derive(Debug)]
pub struct HeartBeat {
    pub value: u16,
}

impl ToBytes for HeartBeat {
    const OPCODE: u8 = Codes::HeartBeat as _;

    fn write_payload(&self, bytes: &mut Vec<u8>) {
        bytes.extend_from_slice(&self.value.to_le_bytes());
    }
}
