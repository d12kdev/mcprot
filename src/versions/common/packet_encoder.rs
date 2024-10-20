use std::io::Write;

use color_eyre::eyre::Result;
use flate2::{write::ZlibEncoder, Compression};

use crate::{errors::PacketEncoderError, types::{packet::{ClientPacket, Packet}, ByteBuffer, VarInt}};

use super::packet::PacketCodecSettings;

#[derive(Debug)]
pub struct PacketEncoder {
    settings: PacketCodecSettings
}

impl PacketEncoder {
    pub fn new(settings: Option<PacketCodecSettings>) -> Self {
        Self {
            settings: settings.unwrap_or_default()
        }
    }

    pub fn set_settings(&mut self, new_settings: &PacketCodecSettings) {
        self.settings = *new_settings;
    }

    pub fn get_settings(&self) -> &PacketCodecSettings {
        &self.settings
    }

    pub fn encode<P: ClientPacket + Packet>(&self, packet: P) -> Result<ByteBuffer, PacketEncoderError> {

        let mut buffer = ByteBuffer::new();

        if self.settings.compress {
            if self.settings.compression_threshold.is_none() {
                return Err(PacketEncoderError::CompressionThresholdNotSet);
            }

            let threshold = self.settings.compression_threshold.unwrap() as usize;
            let packet_id = VarInt::new(P::PACKET_ID);
            let payload = packet.get_payload();
            let size = packet_id.len() + payload.len();

            if size >= threshold {

                // COMPRESS

                let mut combined_data = ByteBuffer::new();
                combined_data.put_varint(packet_id);
                combined_data.put_slice(payload.to_u8());

                let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
                encoder
                    .write_all(combined_data.to_u8())
                    .map_err(PacketEncoderError::CompressionError)?;
                
                let compressed_data = encoder.finish().map_err(PacketEncoderError::CompressionError)?;

                buffer.put_varint(VarInt::from(compressed_data.len() + VarInt::from(size).len()));
                buffer.put_varint(VarInt::from(size));
                buffer.put_slice(&compressed_data);
            } else {
                
                // DON'T COMPRESS

                buffer.put_varint(VarInt::from(size));
                buffer.put_varint(VarInt::new(0));
                buffer.put_varint(packet_id);
                buffer.put_slice(payload.to_u8());
            }

        } else {

            // NORMAL PACKET FORMAT

            let mut packet_buffer = ByteBuffer::new();

            packet_buffer.put_varint(VarInt::new(P::PACKET_ID));
            packet_buffer.put_slice(packet.get_payload().to_u8());

            buffer.put_varint(VarInt::from(packet_buffer.len()));
            buffer.put_slice(packet_buffer.to_u8());
        }

        Ok(buffer)
    }
}

impl Default for PacketEncoder {
    fn default() -> Self {
        Self {
            settings: PacketCodecSettings::default()
        }
    }
}