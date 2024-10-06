use crate::{types::{packet::{Packet, ServerPacket}, VarInt}, versions::common::ConnectionState};

#[allow(unused)]
#[derive(Debug)]
pub struct Handshake {
    pub protocol_version: VarInt,
    pub server_address: String, // String(255)
    pub server_port: u16,
    pub next_state: ConnectionState
}

impl Packet for Handshake {
    const PACKET_ID: i32 = 0x00;
}

impl ServerPacket for Handshake {
    fn read(bytebuffer: &mut crate::types::ByteBuffer) -> color_eyre::eyre::Result<Self> {
        Ok(Self { 
            protocol_version: bytebuffer.get_varint().unwrap(),
            server_address: bytebuffer.get_string_maxsize(255).unwrap(),
            server_port: bytebuffer.get_u16(),
            next_state: bytebuffer.get_varint().unwrap().into()
        })
    }
}