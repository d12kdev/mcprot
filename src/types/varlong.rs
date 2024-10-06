use bytes::{Buf, BufMut, BytesMut};
use color_eyre::eyre::Result;

use crate::{consts::encoding::*, errors::VarLongDecoderError};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct VarLong {
    value: i64
}

impl VarLong {
    pub fn new(value: i64) -> Self {
        VarLong { value: value }
    }

    pub fn decode(buffer: &mut BytesMut) -> Result<Self, VarLongDecoderError> {
        let mut value = 0;
        let mut position = 0;

        while !buffer.is_empty() {
            let current_byte = buffer.get_u8();

            value |= ((current_byte & SEGMENT_BITS) as i64) << position;

            if(current_byte & CONTINUE_BIT) == 0 {
                return Ok(VarLong::new(value));
            }

            position += 7;

            if position >= 64 {
                return Err(VarLongDecoderError::VarLongTooBig);
            }
        }

        return Err(VarLongDecoderError::UnexpectedEndOfBuffer);
    }

    pub fn encode(&mut self, buffer: &mut BytesMut) {
        let mut value = self.value as u64;

        loop {
            if (value & !(SEGMENT_BITS as u64)) == 0 {
                buffer.put_u8(value as u8);
                break;
            }

            buffer.put_u8(((value & (SEGMENT_BITS as u64)) | (CONTINUE_BIT as u64)) as u8);

            value >>= 7;
        }
    }
}

impl From<i64> for VarLong {
    fn from(value: i64) -> Self {
        VarLong { value: value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let mut buf = BytesMut::new();
        let mut value = VarLong::new(300);
        value.encode(&mut buf);
        assert_eq!(buf.as_ref(), &[0xAC, 0x02]);
    }

    #[test]
    fn test_decode() {
        let mut buf = BytesMut::from(&[0xAC, 0x02][..]);
        let value = VarLong::decode( &mut buf).unwrap();
        assert_eq!(value, VarLong::new(300));
    }
}