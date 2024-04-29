use base64::{engine::general_purpose::URL_SAFE_NO_PAD, read::DecoderReader, Engine};
use clap::Parser;
use enum_dispatch::enum_dispatch;
use std::{fmt::Display, path::PathBuf, str::FromStr};
use tokio::fs;

use crate::{
    get_content, get_reader, parse_input_file, process_decrypt, process_encrypt, process_generate,
    process_sign, process_verify, verify_dir, CmdExcutor,
};
use anyhow::Result;

#[derive(Debug, Parser)]
pub struct TextOpts {
    #[command(subcommand)]
    pub subcmd: TextSubCommand,
}

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExcutor)]
pub enum TextSubCommand {
    #[command(name = "sign", about = "Sign text with private/shared key.")]
    Sign(TextSignOpts),
    #[command(name = "verify", about = "Verify text with public/shared key.")]
    Verify(TextVerifyOpts),
    #[command(name = "generate", about = "Generate random key.")]
    Generate(TextKeyGenerateOpts),
    #[command(name = "encrypt", about = "Encrypt text with public key.")]
    Encrypt(TextEncryptOpts),
    #[command(name = "decrypt", about = "Decrypt text with private key.")]
    Decrypt(TextDecryptOpts),
}

#[derive(Debug, Parser)]
pub struct TextKeyGenerateOpts {
    #[arg(long, value_parser=TextSignFormat::from_str, default_value="blake3", help = "key file path, or '-' for stdin")]
    pub format: TextSignFormat,
    #[arg(short, long,  value_parser=verify_dir)]
    pub output: PathBuf,
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short, long, value_parser=parse_input_file, default_value="-", help = "input file path, or '-' for stdin")]
    pub input: String,
    #[arg(short, long, value_parser=parse_input_file, help = "key file path, or '-' for stdin")]
    pub key: String,
    #[arg(long, value_parser=TextSignFormat::from_str, default_value="blake3", help = "key file path, or '-' for stdin")]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short, long, default_value = "-", value_parser=parse_input_file, help = "input file path, or '-' for stdin")]
    pub input: String,
    #[arg(short, long, value_parser=parse_input_file, help = "key file path, or '-' for stdin")]
    pub key: String,
    #[arg(short, long, required = true, help = "signature")]
    pub sig: String,
    #[arg(long, value_parser=TextSignFormat::from_str, default_value="blake3", help = "key file path, or '-' for stdin")]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextEncryptOpts {
    #[arg(short, long, value_parser=parse_input_file, default_value="-", help = "input file path, or '-' for stdin")]
    pub input: String,
    #[arg(short, long, value_parser=parse_input_file, help = "key file path, or '-' for stdin")]
    pub key: String,
    #[arg(short, long, value_parser=parse_input_file, help = "key file path, or '-' for stdin")]
    pub nonce: String,
}

#[derive(Debug, Parser)]
pub struct TextDecryptOpts {
    #[arg(short, long, value_parser=parse_input_file, default_value="-", help = "input file path, or '-' for stdin")]
    pub input: String,
    #[arg(short, long, value_parser=parse_input_file, help = "key file path, or '-' for stdin")]
    pub key: String,
    #[arg(short, long, value_parser=parse_input_file, help = "key file path, or '-' for stdin")]
    pub nonce: String,
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            v => Err(anyhow::anyhow!("Invalid TextSignFormat: {}", v)),
        }
    }
}

impl From<TextSignFormat> for &'static str {
    fn from(f: TextSignFormat) -> Self {
        match f {
            TextSignFormat::Blake3 => "Blake3",
            TextSignFormat::Ed25519 => "Ed25519",
        }
    }
}

impl Display for TextSignFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

impl CmdExcutor for TextSignOpts {
    async fn execute(self) -> Result<()> {
        let mut reader = get_reader(&self.input)?;
        let key = get_content(&self.key)?;
        let signed = process_sign(&mut reader, &key, self.format)?;
        let encode = URL_SAFE_NO_PAD.encode(signed);
        println!("{}", encode);
        Ok(())
    }
}

impl CmdExcutor for TextVerifyOpts {
    async fn execute(self) -> Result<()> {
        let mut reader = get_reader(&self.input)?;
        let key = get_content(&self.key)?;
        let sig = URL_SAFE_NO_PAD.decode(&self.sig)?;
        let verified = process_verify(&mut reader, &key, &sig, self.format)?;
        if verified {
            println!("✓ Signature verified");
        } else {
            println!("⚠ Signature not verified");
        }
        Ok(())
    }
}

impl CmdExcutor for TextKeyGenerateOpts {
    async fn execute(self) -> Result<()> {
        let keys = process_generate(self.format)?;
        for (k, v) in keys {
            fs::write(self.output.join(k), &v).await?;
        }
        Ok(())
    }
}

impl CmdExcutor for TextEncryptOpts {
    async fn execute(self) -> Result<()> {
        let mut reader = get_reader(&self.input)?;
        let key = get_content(&self.key)?;
        let nonce = get_content(&self.nonce)?;
        let encrypted = process_encrypt(&mut reader, &key, &nonce)?;
        println!("{}", URL_SAFE_NO_PAD.encode(encrypted));
        Ok(())
    }
}

impl CmdExcutor for TextDecryptOpts {
    async fn execute(self) -> Result<()> {
        let reader = get_reader(&self.input)?;
        let mut reader = DecoderReader::new(reader, &URL_SAFE_NO_PAD);
        let key = get_content(&self.key)?;
        let nonce = get_content(&self.nonce)?;
        let decrypted = process_decrypt(&mut reader, &key, &nonce)?;
        println!("{}", String::from_utf8(decrypted)?);
        Ok(())
    }
}

impl CmdExcutor for TextOpts {
    async fn execute(self) -> Result<()> {
        self.subcmd.execute().await
    }
}
