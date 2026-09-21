use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("no keyboard device found")]
    NoKeyboardDevice,
    
    #[error("no device matched selector: {0}")]
    DeviceNotFound(String),
}

pub type Result<T> = std::result::Result<T, Error>;