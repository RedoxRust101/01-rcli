use crate::CmdExecutor;

use super::{verify_file, verify_path};
use anyhow::{Ok, Result};
use clap::Parser;
use enum_dispatch::enum_dispatch;
use std::{fmt, fs, path::PathBuf, str::FromStr};

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
pub enum TextSubCommand {
    #[command(about = "Sign a message with a private/shared key")]
    Sign(TextSignOpts),
    #[command(about = "Verify a signed message with a public/shared key")]
    Verify(TextVerifyOpts),
    #[command(about = "Generate a private/public key pair")]
    Generate(TextKeyGenerateOpts),
    #[command(about = "Encrypt a message with a shared key")]
    Encrypt(TextEncryptOpts),
    #[command(about = "Decrypt a message with a shared key")]
    Decrypt(TextDecryptOpts),
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(short, long, value_parser = parse_format, default_value = "blake3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(short, long, value_parser = parse_format, default_value = "blake3")]
    pub format: TextSignFormat,
    #[arg(long)]
    pub signature: String,
}

#[derive(Debug, Parser)]
pub struct TextKeyGenerateOpts {
    #[arg(short, long, value_parser = parse_format, default_value = "blake3")]
    pub format: TextSignFormat,
    #[arg(short, long, value_parser = verify_path)]
    pub output: PathBuf,
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
    Chacha20,
}

#[derive(Debug, Parser)]
pub struct TextEncryptOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
}

#[derive(Debug, Parser)]
pub struct TextDecryptOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
}

fn parse_format(format: &str) -> Result<TextSignFormat, anyhow::Error> {
    format.parse()
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(Self::Blake3),
            "ed25519" => Ok(Self::Ed25519),
            "chacha20" => Ok(Self::Chacha20),
            _ => Err(anyhow::anyhow!("Invalid format: {}", s)),
        }
    }
}

impl From<TextSignFormat> for &'static str {
    fn from(format: TextSignFormat) -> &'static str {
        match format {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
            TextSignFormat::Chacha20 => "chacha20",
        }
    }
}

impl fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

impl CmdExecutor for TextSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let sign = crate::process_text_sign(&self.input, &self.key, self.format).await?;
        println!("{}", sign);
        Ok(())
    }
}

impl CmdExecutor for TextVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let ret = crate::process_text_verify(&self.input, &self.key, self.format, &self.signature)
            .await?;
        println!("{}", ret);
        Ok(())
    }
}

impl CmdExecutor for TextKeyGenerateOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let key = crate::process_text_key_generate(self.format)?;
        match self.format {
            crate::TextSignFormat::Blake3 => {
                let name = self.output.join("blake3.txt");
                fs::write(name, &key[0])?;
            }
            crate::TextSignFormat::Ed25519 => {
                let name = &self.output;
                fs::write(name.join("ed25519.sk"), &key[0])?;
                fs::write(name.join("ed25519.pk"), &key[1])?;
            }
            crate::TextSignFormat::Chacha20 => {
                let name = &self.output;
                fs::write(name.join("chacha20.key"), &key[0])?;
            }
        }
        Ok(())
    }
}

impl CmdExecutor for TextEncryptOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let ciphertext = crate::process_text_encrypt(&self.input, &self.key).await?;
        println!("\nciphertext: {}", ciphertext);
        Ok(())
    }
}

impl CmdExecutor for TextDecryptOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let plaintext = crate::process_text_decrypt(&self.input, &self.key).await?;
        println!("\nplaintext: {}", plaintext);
        Ok(())
    }
}
