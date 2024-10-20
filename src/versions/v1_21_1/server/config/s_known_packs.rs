use color_eyre::eyre::Result;

use crate::types::{packet::{Packet, ServerPacket}, ByteBuffer, VarInt};

#[derive(Debug)]
#[allow(unused)]
struct SKPPack {
    pub namespace: String,
    pub id: String,
    pub version: String
}

impl SKPPack {
    pub fn decode(buff: &mut ByteBuffer) -> Result<Self> {
        Ok(
            Self {
                namespace: buff.get_string()?,
                id: buff.get_string()?,
                version: buff.get_string()?
            }
        )
    }
}

#[derive(Debug)]
#[allow(private_interfaces)]
pub struct ServerboundKnownPacks {
    pub known_pack_count: VarInt,
    pub known_packs: Vec<SKPPack>
}

impl Packet for ServerboundKnownPacks {
    const PACKET_ID: i32 = 0x07;
}

impl ServerPacket for ServerboundKnownPacks {
    fn decode(bytebuffer: &mut crate::types::ByteBuffer) -> color_eyre::eyre::Result<Self> {
        let pack_count = bytebuffer.get_varint()?.get_value();
        let mut packs: Vec<SKPPack> = Vec::with_capacity(pack_count as usize);

        for _ in 0..pack_count {
            let pack = SKPPack::decode(bytebuffer)?;
            packs.push(pack);
        }

        Ok(
            Self {
                known_pack_count: VarInt::new(pack_count),
                known_packs: packs
            }
        )
    }
}