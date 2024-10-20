use bytes::{Buf, BufMut, BytesMut};
use color_eyre::eyre::Result;
use serde::Serialize;

use crate::{consts::encoding::*, errors::VarLongDecoderError};

/// Variable-length data encoding a two's complement signed 64-bit integer; <https://wiki.vg/Data_types#VarInt_and_VarLong>
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize)]
pub struct VarLong {
    value: i64
}

impl VarLong {
    pub fn new(value: i64) -> Self {
        VarLong { value: value }
    }

    pub fn get_value(&self) -> i64 {
        self.value
    }

    pub fn len(&self) -> usize {
        let mut buffer = BytesMut::new();
        self.encode(&mut buffer);

        buffer.len()
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

    pub fn encode(&self, buffer: &mut BytesMut) {
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

impl From<usize> for VarLong {
    fn from(value: usize) -> Self {
        VarLong { value: (value as i64) }
    }
}


impl Into<i64> for VarLong {
    fn into(self) -> i64 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let mut buf = BytesMut::new();
        let value = VarLong::new(300);
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