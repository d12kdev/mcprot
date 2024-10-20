mod connstate;
pub use connstate::*;

mod status;
pub use status::*;

mod identifier;
pub use identifier::*;

mod chat_mode;
pub use chat_mode::*;

mod main_hand;
pub use main_hand::*;

mod rpp_result;
pub use rpp_result::*;

mod packet_encoder;
mod packet_decoder;

pub mod packet {

    #[derive(Debug, Clone, Copy)]
    pub struct PacketCodecSettings {
        pub compress: bool,
        pub compression_threshold: Option<i32>
    }

    impl PacketCodecSettings {
        pub fn new(compress: bool, compression_threshold: Option<i32>) -> Self {
            Self {
                compress,
                compression_threshold
            }
        }


        pub fn allow_compression(&mut self, threshold: i32) {
            self.compress = true;
            self.compression_threshold = Some(threshold);
        }

        pub fn disable_compression(&mut self) {
            self.compress = false;
        }
    }

    impl Default for PacketCodecSettings {
        fn default() -> Self {
            Self {
                compress: false,
                compression_threshold: None
            }
        }
    }

    pub use super::packet_encoder::*;
    pub use super::packet_decoder::*;

    #[cfg(test)]
    mod tests {
        use crate::protocol::client::status::PongResponse;

        use super::{PacketDecoder, PacketEncoder};

        #[test]
        fn encode() {
            let packet = PongResponse::new(14723);

            let encoder = PacketEncoder::new(None);

            let encoded = encoder.encode(packet).unwrap();
            
            let correct: &[u8] = &[9, 1, 0, 0, 0, 0, 0, 0, 57, 131];
            assert_eq!(encoded.to_u8(), correct);
        }

        #[test]
        fn decode() {
            let packet = PongResponse::new(64);

            let encoder = PacketEncoder::new(None);
            let decoder = PacketDecoder::new(None);
            let mut encoded = encoder.encode(packet).unwrap();
            
            let mut result = decoder.decode(&mut encoded).unwrap();

            let result = result.payload.get_i64();

            assert_eq!(result, 64);
        }
    }
}

pub mod text;