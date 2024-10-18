use std::io::{Cursor, Read, Write};

use color_eyre::eyre::Result;
use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};

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


/// Function used to get packet id and its payload (compressed) from the ByteBuffer
/// Note: I don't know what i'm doing rn, im tired - but it "works" (tests), i don't know if i was reading the docs well
pub fn decode_packet_compressed(buffer: &mut ByteBuffer) -> Result<RawPacket> {
    let _packet_len = buffer.get_varint()?.get_value();
    let size = buffer.get_varint()?.get_value();

    let packet_id: i32;
    let payload: ByteBuffer;

    if size > 0 {
        // COMPRESSED
        let mut decoder = ZlibDecoder::new(Cursor::new(buffer.get_bytesmut()));
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed)?;

        let mut decompressed = ByteBuffer::from_u8buffer(&decompressed);

        packet_id = decompressed.get_varint()?.get_value();
        payload = decompressed.get_slice();
    } else {
        // NOT COMPRESSED
        packet_id = buffer.get_varint()?.get_value();
        payload = ByteBuffer::from_buffer(buffer.get_bytesmut());
    }

    Ok(
        RawPacket {
            packet_id: packet_id,
            payload: payload
        }
    )
}

#[cfg(test)]
mod tests {
    use crate::{common::packet::decode_packet_compressed, types::{packet::RawPacket, ByteBuffer, VarInt}};

    use super::encode_packet_compress;


    #[test]
    fn encode_decode_compressed() {
        let mut buffer = ByteBuffer::new();
        buffer.put_varint(VarInt::new(33));
        let mut packet = encode_packet_compress(0x00, buffer.clone(), VarInt::new(1));
        let de_packet = decode_packet_compressed(&mut packet).unwrap();

        assert_eq!(de_packet, RawPacket {
            packet_id: 0x00,
            payload: buffer
        })
    }
}