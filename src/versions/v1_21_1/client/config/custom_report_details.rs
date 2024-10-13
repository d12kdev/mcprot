use crate::{common::packet::encode_packet, types::{packet::{ClientPacket, Packet}, ByteBuffer, VarInt}};

#[derive(Debug)]
struct CRPDetail {
    title: String,
    description: String
}

impl CRPDetail {
    pub fn encode(&self) -> ByteBuffer {
        let mut buffer = ByteBuffer::new();
        buffer.put_string(self.title.clone()).unwrap();
        buffer.put_string(self.description.clone()).unwrap();

        buffer
    }
}

#[derive(Debug)]
pub struct CustomReportDetails {
    details_count: VarInt,
    details: Vec<CRPDetail>
}

impl Packet for CustomReportDetails {
    const PACKET_ID: i32 = 0x0F;
}

impl CustomReportDetails {
    #[allow(private_interfaces)]
    pub fn new_detail(title: String, description: String) -> CRPDetail {
        CRPDetail { title: title, description: description }
    }

    #[allow(private_interfaces)]
    pub fn new(details: Vec<CRPDetail>) -> Self {
        Self { details_count: VarInt::from(details.len() as i32), details }
    }
}

impl ClientPacket for CustomReportDetails {
    fn write(&self) -> ByteBuffer {
        let mut buffer = ByteBuffer::new();
        buffer.put_varint(self.details_count);

        for detail in &self.details {
            buffer.put_slice(detail.encode().to_u8());
        }

        encode_packet(Self::PACKET_ID, buffer)
    }
}