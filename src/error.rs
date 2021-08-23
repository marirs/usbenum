#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("USB enumeration parsing error")]
    UsbParsingError,
    #[error("Failed IOServiceGetMatchingServices")]
    FailedIoService,
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("Unknown Error")]
    Unknown,
    #[error("{0}")]
    Generic(String),
    #[error("Not implemented")]
    NotImplemented,
}
