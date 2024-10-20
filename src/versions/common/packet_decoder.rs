use std::io::{Cursor, Read};

use color_eyre::eyre::Result;
use flate2::read::ZlibDecoder;

use crate::{errors::PacketDecoderError, types::{packet::RawPacket, ByteBuffer, VarInt}};

use super::packet::PacketCodecSettings;

#[derive(Debug)]
pub struct PacketDecoder {
    settings: PacketCodecSettings
}

impl PacketDecoder {
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

    pub fn decode(&self, buffer: &mut ByteBuffer) -> Result<RawPacket, PacketDecoderError> {

        let result: RawPacket;

        fn get_varint_wrap(buf: &mut ByteBuffer) -> Result<VarInt, PacketDecoderError> {
            Ok(
                buf.get_varint().map_err(PacketDecoderError::VarIntDecodingError)?
            )
        }

        if self.settings.compress {
            if self.settings.compression_threshold.is_none() {
                return Err(PacketDecoderError::CompressionThresholdNotSet);
            }

            let _packet_len = get_varint_wrap(buffer)?;
            let size = get_varint_wrap(buffer)?.get_value();

            let packet_id: i32;
            let payload: ByteBuffer;

            if size > 0 {

                // COMPRESSED

                let mut decoder = ZlibDecoder::new(Cursor::new(buffer.get_bytesmut()));
                let mut data = Vec::new();
                decoder.read_to_end(&mut data).map_err(PacketDecoderError::DecompressionError)?;
            
                let mut data = ByteBuffer::from_u8buffer(&data);

                packet_id = get_varint_wrap(&mut data)?.get_value();
                payload = ByteBuffer::from_buffer(buffer.get_bytesmut());
            } else {
                
                // NOT COMPRESSED

                packet_id = get_varint_wrap(buffer)?.get_value();
                payload = ByteBuffer::from_buffer(buffer.get_bytesmut());
            }

            result = RawPacket {
                packet_id,
                payload
            }
        } else {
            let _length = get_varint_wrap(buffer)?;
            let packet_id = get_varint_wrap(buffer)?.get_value();
            let payload = ByteBuffer::from_buffer(buffer.get_bytesmut());

            result = RawPacket {
                packet_id,
                payload
            }
        }

        Ok(result)
    }
}