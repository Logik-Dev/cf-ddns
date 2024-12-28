mod cli;
mod error;
mod http;

pub use self::error::{Error, Result};

#[tokio::main]
async fn main() -> Result<()> {
    match cli::parse() {
        Ok(params) => http::update_dns_record(params).await,
        Err(e) => return Err(e)?,
    }
}
