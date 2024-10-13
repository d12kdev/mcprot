use color_eyre::eyre::Result;

use crate::types::{packet::RawPacket, ByteBuffer, VarInt};

/// Function to encode packet id and it's payload to ByteBuffer
pub fn encode_packet(packet_id: i32, payload: ByteBuffer) -> ByteBuffer {
    let mut buffer = ByteBuffer::new();

    let packet_id = VarInt::new(packet_id);
    buffer.put_varint(packet_id);

    buffer.put_slice(&payload.get_bytesmut());

    let total_len = buffer.len();
    let total_len = VarInt::new(total_len as i32);

    let mut final_buffer = ByteBuffer::new();
    final_buffer.put_varint(total_len);
    final_buffer.put_slice(&buffer.get_bytesmut());

    return final_buffer;
}

/// Function used to get packet id and it's payload from the ByteBuffer
pub fn decode_packet(buffer: &mut ByteBuffer) -> Result<RawPacket> {
    let _len = buffer.get_varint().unwrap();

    let packet_id: i32 = buffer.get_varint().unwrap().into();
    let payload = buffer.get_bytesmut();

    Ok(
        RawPacket {
            packet_id: packet_id,
            payload: ByteBuffer::from_buffer(payload)
        }
    )
}