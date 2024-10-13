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