mod base64;
mod csv;
mod genpass;
mod http;
mod jwt;
mod text;
use std::path::{Path, PathBuf};

pub use self::{base64::*, csv::*, genpass::*, http::*, jwt::*, text::*};

use clap::Parser;
use enum_dispatch::enum_dispatch;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate random password")]
    GenPass(GenPassOpts),
    #[command(subcommand, name = "base64", about = "Base64 encode/decode")]
    Base64(Base64SubCommand),
    #[command(subcommand, name = "text", about = "Text sign/verify")]
    Text(TextSubCommand),
    #[command(subcommand, name = "http", about = "HTTP server")]
    Http(HttpSubCommand),
    #[command(subcommand, name = "jwt", about = "JWT sign/verify")]
    Jwt(JwtSubCommand),
}

fn verify_file(filename: &str) -> anyhow::Result<String, &'static str> {
    // if input is "-" or file exists
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File dose not exists")
    }
}

fn verify_path(path: &str) -> anyhow::Result<PathBuf, &'static str> {
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("Path dose not exists or is not a directory")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("-"), Ok("-".into()));
        assert_eq!(verify_file("*"), Err("File dose not exists"));
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_file("not-exists"), Err("File dose not exists"));
    }
}
