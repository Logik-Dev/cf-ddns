use std::{
    fs::{self},
    path::PathBuf,
};

use crate::{Error, Result};
use args::CloudflareArgs;
use clap::Parser;
pub mod args;

#[derive(Debug)]
pub struct Parameters {
    pub domain: String,
    pub email: String,
    pub token: String,
}

impl TryFrom<CloudflareArgs> for Parameters {
    type Error = Error;

    fn try_from(args: CloudflareArgs) -> Result<Self> {
        let domain = match args.domain.domain_inline {
            Some(d) => Ok(d),
            None => read_file(args.domain.domain_file.unwrap()),
        }?;

        let email = match args.email.email_inline {
            Some(e) => Ok(e),
            None => read_file(args.email.email_file.unwrap()),
        }?;

        let token = match args.token.token_inline {
            Some(t) => Ok(t),
            None => read_file(args.token.token_file.unwrap()),
        }?;

        Ok(Self {
            domain,
            email,
            token,
        })
    }
}

pub fn parse() -> Result<Parameters> {
    Parameters::try_from(args::CloudflareArgs::parse())
}

fn read_file(path: PathBuf) -> Result<String> {
    if !path.exists() {
        return Err(Error::FileNotFound(path));
    }
    fs::read_to_string(&path).map(|mut content| {
        if content.is_empty() {
            Err(Error::FileIsEmpty(path))
        } else {
            content.pop();
            Ok(content)
        }
    })?
}
