use thiserror::Error;

/// Errors that can happen when decoding VarInt
#[derive(Error, Debug)]
pub enum VarIntDecoderError {
    #[error("VarInt is too big")]
    VarIntTooBig,
    #[error("Unexpected end of buffer")]
    UnexpectedEndOfBuffer
}

/// Errors that can happen when decoding VarLong
#[derive(Error, Debug)]
pub enum VarLongDecoderError {
    #[error("VarLong is too big")]
    VarLongTooBig,
    #[error("Unexpected end of buffer")]
    UnexpectedEndOfBuffer
}

/// Errors that can happen when decoding String
#[derive(Error, Debug)]
pub enum StringDecoderError {
    #[error("String len is bigger than max_size")]
    StringTooBig,
    #[error("UTF-8 Conversion error: {0}")]
    Utf8Error(String)
}

#[derive(Error, Debug)]
pub enum PacketEncoderError {
    #[error("Compression threshold not set")]
    CompressionThresholdNotSet,
    #[error("Compression Error: {0}")]
    CompressionError(#[from] std::io::Error)
}

#[derive(Error, Debug)]
pub enum PacketDecoderError {
    #[error("Compression threshold not set")]
    CompressionThresholdNotSet,
    #[error("Decompression Error: {0}")]
    DecompressionError(#[from] std::io::Error),
    #[error("Error while decoding: {0}")]
    VarIntDecodingError(#[from] VarIntDecoderError)
}