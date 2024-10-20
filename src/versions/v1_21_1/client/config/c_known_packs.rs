use crate::types::{packet::{ClientPacket, Packet}, ByteBuffer, VarInt};

#[derive(Debug)]
struct CKPPack {
    namespace: String,
    id: String,
    version: String
}

impl CKPPack {
    pub fn encode(&self) -> ByteBuffer {
        let mut buffer = ByteBuffer::new();
        buffer.put_string(self.namespace.clone()).unwrap();
        buffer.put_string(self.id.clone()).unwrap();
        buffer.put_string(self.version.clone()).unwrap();

        buffer
    }
}

#[derive(Debug)]
pub struct ClientboundKnownPacks {
    known_pack_count: VarInt,
    known_packs: Vec<CKPPack>
}

impl Packet for ClientboundKnownPacks {
    const PACKET_ID: i32 = 0x0E;
}

impl ClientboundKnownPacks {
    #[allow(private_interfaces)]
    pub fn new_pack(namespace: String, id: String, version: String) -> CKPPack {
        CKPPack { namespace: namespace, id: id, version: version }
    }

    #[allow(private_interfaces)]
    pub fn new(packs: Vec<CKPPack>) -> Self {
        Self { known_pack_count: VarInt::from(packs.len() as i32), known_packs: packs }
    }
}

impl ClientPacket for ClientboundKnownPacks {
    fn get_payload(&self) -> ByteBuffer {
        let mut buffer = ByteBuffer::new();
        buffer.put_varint(self.known_pack_count);

        for pack in &self.known_packs {
            buffer.put_slice(pack.encode().to_u8());
        }

        buffer
    }
}