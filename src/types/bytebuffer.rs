use core::str;

use bytes::{Buf, BufMut, BytesMut};
use color_eyre::eyre::Result;

use crate::{common::text::TextComponent, errors::{StringDecoderError, VarIntDecoderError, VarLongDecoderError}};

use super::{VarInt, VarLong};

#[derive(Debug, Clone)]
pub struct ByteBuffer {
    buffer: BytesMut
}

impl ByteBuffer {
    pub fn new() -> Self {
        Self { buffer: BytesMut::new() }
    }

    pub fn from_buffer(buffer: BytesMut) -> Self {
        Self { buffer: buffer }
    }

    pub fn from_u8buffer(src: &[u8]) -> Self {
        Self {
            buffer: BytesMut::from(src)
        }
    }

    pub fn len(&mut self) -> usize {
        self.buffer.len()
    }

    pub fn get_u8buffer(&self) -> &[u8] {
        return self.buffer.as_ref();
    }
    pub fn to_u8(&self) -> &[u8] {
        return self.get_u8buffer();
    }

    pub fn get_bytesmut(&self) -> BytesMut {
        return self.buffer.clone();
    }
}

impl ByteBuffer {
    pub fn put_slice(&mut self, src: &[u8]) {
        self.buffer.put_slice(src);
    }


    pub fn put_textcomponent(&mut self, value: TextComponent) {
        let component_json = serde_json::to_string(&value).unwrap();
        self.put_string(component_json).unwrap();
    }


    pub fn put_varint(&mut self, value: VarInt) {
        value.encode(&mut self.buffer);
    }

    pub fn get_varint(&mut self) -> Result<VarInt, VarIntDecoderError> {
        VarInt::decode(&mut self.buffer)
    }

    
    pub fn put_varlong(&mut self, value: &mut VarLong) {
        value.encode(&mut self.buffer);
    }

    pub fn get_varlong(&mut self) -> Result<VarLong, VarLongDecoderError> {
        VarLong::decode(&mut self.buffer)
    }

    pub fn put_string(&mut self, value: String) -> Result<(), StringDecoderError> {
        let utf16_len = value.encode_utf16().count();
        

        if utf16_len > (i16::MAX as usize) {
            return Err(StringDecoderError::StringTooBig);
        }

        let utf8_bytes = value.as_bytes();

        let varint_len = VarInt::new(utf16_len as i32);
        self.put_varint(varint_len.clone());

        self.buffer.put_slice(utf8_bytes);

        Ok(())
    }

    pub fn get_string_maxsize(&mut self, max_size: i32) -> Result<String, StringDecoderError> {
        let size: i32 = self.get_varint().unwrap().into();
        if size > max_size {
            return Err(StringDecoderError::StringTooBig);
        }

        let text = self.copy_to_bytes(size as usize).unwrap();
        if text.len() as i32 > max_size {
            return Err(StringDecoderError::StringTooBig);
        }

        match str::from_utf8(&text) {
            Ok(result) => Ok(result.to_string()),
            Err(e) => Err(StringDecoderError::Utf8Error(e.to_string()))
        }
    }

    pub fn get_string(&mut self) -> Result<String, StringDecoderError> {
        self.get_string_maxsize(i16::MAX as i32)
    }



    pub fn copy_to_bytes(&mut self, len: usize) -> Result<bytes::Bytes, ()> {
        if self.buffer.len() >= len {
            Ok(self.buffer.copy_to_bytes(len))
        } else {
            Err(())
        }
    }
}

// BASIC TYPES
impl ByteBuffer {

    //
    //  PUT
    //

    pub fn put_bool(&mut self, value: bool) {
        self.buffer.put_u8(if value { 1 } else { 0 });
    }

    // SIGNED

    pub fn put_i8(&mut self, value: i8) {
        self.buffer.put_i8(value);
    }

    pub fn put_i16(&mut self, value: i16) {
        self.buffer.put_i16(value);
    }

    pub fn put_i32(&mut self, value: i32) {
        self.buffer.put_i32(value);
    }

    pub fn put_i64(&mut self, value: i64) {
        self.buffer.put_i64(value);
    }

    pub fn put_i128(&mut self, value: i128) {
        self.buffer.put_i128(value);
    }

    // USIGNED

    pub fn put_u8(&mut self, value: u8) {
        self.buffer.put_u8(value);
    }

    pub fn put_u16(&mut self, value: u16) {
        self.buffer.put_u16(value);
    }

    pub fn put_u32(&mut self, value: u32) {
        self.buffer.put_u32(value);
    }

    pub fn put_u64(&mut self, value: u64) {
        self.buffer.put_u64(value);
    }

    pub fn put_u128(&mut self, value: u128) {
        self.buffer.put_u128(value);
    }

    //
    //  GET
    //

    pub fn get_bool(&mut self) -> bool {
        self.get_u8() != 0
    }

    // SIGNED

    pub fn get_i8(&mut self) -> i8 {
        self.buffer.get_i8()
    }

    pub fn get_i16(&mut self) -> i16 {
        self.buffer.get_i16()
    }

    pub fn get_i32(&mut self) -> i32 {
        self.buffer.get_i32()
    }

    pub fn get_i64(&mut self) -> i64 {
        self.buffer.get_i64()
    }

    pub fn get_i128(&mut self) -> i128 {
        self.buffer.get_i128()
    }

    // UNSIGNED

    pub fn get_u8(&mut self) -> u8 {
        self.buffer.get_u8()
    }

    pub fn get_u16(&mut self) -> u16 {
        self.buffer.get_u16()
    }

    pub fn get_u32(&mut self) -> u32 {
        self.buffer.get_u32()
    }

    pub fn get_u64(&mut self) -> u64 {
        self.buffer.get_u64()
    }

    pub fn get_u128(&mut self) -> u128 {
        self.buffer.get_u128()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn varint() {
        let original = VarInt::new(2147483647);
        let mut bytebuf = ByteBuffer::new();
        bytebuf.put_varint(original.clone());
        let value = bytebuf.get_varint().unwrap();
        assert_eq!(value, original);
    }

    #[test]
    fn varlong() {
        let original = VarLong::new(9223372036854775807);
        let mut bytebuf = ByteBuffer::new();
        bytebuf.put_varlong(&mut original.clone());
        let value = bytebuf.get_varlong().unwrap();
        assert_eq!(value, original);
    }

    #[test]
    fn bool() {
        let original = true;
        let mut bytebuf = ByteBuffer::new();
        bytebuf.put_bool(original.clone());
        let value = bytebuf.get_bool();
        assert_eq!(value, original);
    }
}