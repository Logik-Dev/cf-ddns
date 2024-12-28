use derive_more::From;
use reqwest::StatusCode;
use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    // -- cli
    FileIsEmpty(PathBuf),
    FileNotFound(PathBuf),

    // -- http
    DnsZoneNotFound,
    DnsRecordNotFound,
    #[from]
    StatusCodeError {
        status: StatusCode,
        body: String,
    },

    // -- external
    #[from]
    Io(std::io::Error),
    #[from]
    Request(reqwest::Error),
    #[from]
    InvalidHeaderName(reqwest::header::InvalidHeaderName),
    #[from]
    InvalidHeaderValue(reqwest::header::InvalidHeaderValue),
    #[from]
    Serde(serde_json::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}
