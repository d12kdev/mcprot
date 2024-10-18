use crate::types::VarInt;


#[derive(Debug)]
pub enum ResourcePackResponseResult {
    SuccessfullyDownloaded,
    Declined,
    FailedToDownload,
    Accepted,
    Downloaded,
    InvalidUrl,
    FailedToReload,
    Discarded
}

impl From<i32> for ResourcePackResponseResult {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::SuccessfullyDownloaded,
            1 => Self::Declined,
            2 => Self::FailedToDownload,
            3 => Self::Accepted,
            4 => Self::Downloaded,
            5 => Self::InvalidUrl,
            6 => Self::FailedToReload,
            7 => Self::Discarded,
            _ => Self::Declined
        }
    }
}

impl From<VarInt> for ResourcePackResponseResult {
    fn from(value: VarInt) -> Self {
        Self::from(value.get_value())
    }
}