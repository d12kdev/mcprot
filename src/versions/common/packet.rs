use std::io::Write;

use color_eyre::eyre::Result;
use flate2::{write::ZlibEncoder, Compression};

use crate::types::{packet::RawPacket, ByteBuffer, VarInt};

/// Function to encode packet id and its payload to ByteBuffer
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

/// Function to encode packet id and its payload, with compression
pub fn encode_packet_compress(packet_id: i32, payload: ByteBuffer, compression_threshold: VarInt) -> ByteBuffer {
    let threshold = compression_threshold.get_value() as usize;

    let mut packet_id_buffer = ByteBuffer::new();
    packet_id_buffer.put_varint(VarInt::from(packet_id));
    let packet_id_len = packet_id_buffer.len();

    let payload_len = payload.len();
    let size = packet_id_len + payload_len;

    let mut buffer = ByteBuffer::new();

    if size >= threshold {
        // Compress packet_id and payload together
        let mut combined_data = ByteBuffer::new();
        combined_data.put_slice(packet_id_buffer.to_u8());
        combined_data.put_slice(payload.to_u8());

        // Compress the combined data (packet_id + payload)
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(combined_data.to_u8()).unwrap();
        let compressed_data = encoder.finish().unwrap();
        let compressed_len = compressed_data.len();

        // Write the packet structure for compressed data
        // packet_length = compressed_len + length of data_len (VarInt size)
        buffer.put_varint(VarInt::from((compressed_len + VarInt::from(size as i32).len()) as i32));
        buffer.put_varint(VarInt::from(size as i32));  // Uncompressed size (VarInt size + payload size)
        buffer.put_slice(&compressed_data);  // Compressed data (packet_id + payload)
    } else {
        // No compression
        buffer.put_varint(VarInt::from(size as i32));  // Packet Length (Uncompressed)
        buffer.put_varint(VarInt::new(0));  // Indicate uncompressed
        buffer.put_slice(packet_id_buffer.to_u8());  // Uncompressed packet ID (VarInt)
        buffer.put_slice(payload.to_u8());  // Uncompressed payload
    }

    buffer
}

/// Function used to get packet id and its payload from the ByteBuffer
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