use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("something occurred")]
    Unknown,
    #[error("string length = {}, expected = 4", .0.len())]
    InvalidStringLength(String),
    #[error("string characters = `{}`, expected a..z, A..Z", .0)]
    InvalidStringCharacter(String),
    #[error("invalid array length = `{}`, expected > 12", .0)]
    ChunkSourceArrayTooShort(usize),
    #[error("chunk length data and actual length mismatch")]
    ChunkLengthMismatch,
    #[error("chunk checksum data invalid")]
    ChunkInvalidChecksum
}