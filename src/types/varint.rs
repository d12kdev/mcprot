use bytes::{Buf, BufMut, BytesMut};
use color_eyre::eyre::Result;
use serde::Serialize;

use crate::{consts::encoding::*, errors::VarIntDecoderError};

/// Variable-length data encoding a two's complement signed 32-bit integer; <https://wiki.vg/Data_types#VarInt_and_VarLong>
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize)]
pub struct VarInt {
    value: i32
}

impl VarInt {

    pub fn new(value: i32) -> Self {
        VarInt { value: value }
    }

    pub fn get_value(&self) -> i32 {
        return self.value;
    }

    pub fn len(&self) -> usize {
        let mut buffer = BytesMut::new();
        self.encode(&mut buffer);

        buffer.len()
    }

    pub fn decode(buffer: &mut BytesMut) -> Result<Self, VarIntDecoderError> {
        let mut value = 0;
        let mut position = 0;

        while !buffer.is_empty() {
            let current_byte = buffer.get_u8();

            value |= ((current_byte & SEGMENT_BITS) as i32) << position;

            if(current_byte & CONTINUE_BIT) == 0 {
                return Ok(VarInt::new(value));
            }

            position += 7;

            if position >= 32 {
                return Err(VarIntDecoderError::VarIntTooBig);
            }
        }

        return Err(VarIntDecoderError::UnexpectedEndOfBuffer);
    }

    pub fn encode(self, buffer: &mut BytesMut) {
        let mut value = self.value as u32;

        loop {
            if (value & !(SEGMENT_BITS as u32)) == 0 {
                buffer.put_u8(value as u8);
                break;
            }

            buffer.put_u8(((value & (SEGMENT_BITS as u32)) | (CONTINUE_BIT as u32)) as u8);
            
            value >>= 7;
        }
    }
}

impl From<i32> for VarInt {
    fn from(value: i32) -> Self {
        VarInt { value: value }
    }
}

impl Into<i32> for VarInt {
    fn into(self) -> i32 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let mut buf = BytesMut::new();
        let value = VarInt::new(300);
        value.encode(&mut buf);
        assert_eq!(buf.as_ref(), &[0xAC, 0x02]);
    }

    #[test]
    fn test_decode() {
        let mut buf = BytesMut::from(&[0xAC, 0x02][..]);
        let value = VarInt::decode( &mut buf).unwrap();
        assert_eq!(value, VarInt::new(300));
    }
}