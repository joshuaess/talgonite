use crate::ToBytes;

use super::Codes;

#[derive(Debug)]
pub enum Click {
    TargetEntity(u32),
    TargetWall { x: u16, y: u16, is_right: bool },
}

impl ToBytes for Click {
    const OPCODE: u8 = Codes::Click as _;

    fn write_payload(&self, bytes: &mut Vec<u8>) {
        match self {
            Click::TargetEntity(id) => {
                bytes.push(1);
                bytes.extend_from_slice(&id.to_be_bytes());
            }
            Click::TargetWall { x, y, is_right } => {
                bytes.push(3);
                bytes.extend_from_slice(&x.to_be_bytes());
                bytes.extend_from_slice(&y.to_be_bytes());
                bytes.push(*is_right as u8);
            }
        }
    }
}
