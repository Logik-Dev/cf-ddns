use std::path::PathBuf;

use clap::{Args, Parser};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct CloudflareArgs {
    #[command(flatten)]
    pub domain: Domain,

    #[command(flatten)]
    pub email: Email,

    #[command(flatten)]
    pub token: Token,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
pub struct Domain {
    #[arg(short, long = "domain", value_name = "DOMAIN", env = "CF_DOMAIN")]
    pub domain_inline: Option<String>,

    #[arg(
        short = 'D',
        long = "domain-file",
        value_name = "PATH",
        env = "CF_DOMAIN_FILE"
    )]
    pub domain_file: Option<PathBuf>,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
pub struct Email {
    #[arg(short = 'e', long = "email", value_name = "EMAIL", env = "CF_EMAIL")]
    pub email_inline: Option<String>,

    #[arg(
        short = 'E',
        long = "email-file",
        value_name = "PATH",
        env = "CF_EMAIL_FILE"
    )]
    pub email_file: Option<PathBuf>,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
pub struct Token {
    #[arg(short = 't', long = "token", value_name = "TOKEN", env = "CF_TOKEN")]
    pub token_inline: Option<String>,

    #[arg(
        short = 'T',
        long = "token-file",
        value_name = "PATH",
        env = "CF_TOKEN_FILE"
    )]
    pub token_file: Option<PathBuf>,
}
