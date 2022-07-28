#[derive(Debug)]
pub enum MediaError {
    DecodingError(String),
    IoError(std::io::Error)
}

impl From<std::io::Error> for MediaError {
    fn from(err: std::io::Error) -> Self {
        MediaError::IoError(err)
    }
}

pub type MediaResult<T> = Result<T, MediaError>;