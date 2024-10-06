use color_eyre::eyre::Result;

use super::ByteBuffer;

#[derive(Debug)]
pub struct RawPacket {
    pub packet_id: i32,
    pub payload: ByteBuffer
}

pub trait Packet {
    const PACKET_ID: i32;
}

pub trait ServerPacket: Packet + Sized {
    fn read(bytebuffer: &mut ByteBuffer) -> Result<Self>;
}

pub trait ClientPacket: Packet {
    fn write(&self) -> ByteBuffer;
}