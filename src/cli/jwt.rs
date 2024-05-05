use std::str::FromStr;

use anyhow::Result;
use clap::Parser;
use enum_dispatch::enum_dispatch;
use jsonwebtoken::{get_current_timestamp, Algorithm, Validation};
use serde_json::json;
use std::collections::HashSet;

use crate::{
    get_content, parse_duration, parse_input_file, process_jwt_sign, process_jwt_verify, CmdExcutor,
};
#[derive(Debug, Parser)]
pub struct JwtOpts {
    #[command(subcommand)]
    pub subcmd: JwtSubCommand,
}

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExcutor)]
pub enum JwtSubCommand {
    #[command(name = "sign", about = "jwt sign")]
    Sign(JwtSignOpts),
    #[command(name = "verify", about = "jwt verify")]
    Verify(JwtVerifyOpts),
}

#[derive(Debug, Parser)]
pub struct JwtSignOpts {
    // jwt sign --sub acme --aud device1 --exp 14d --key key.pem
    #[arg(long, default_value = "HS384", value_parser = Algorithm::from_str, help = "claims algorithm")]
    pub alg: Algorithm,
    #[arg(long, value_parser=parse_input_file, default_value="-", help = "sign key file path, or '-' for stdin")]
    pub key: String,
    // jwt claims
    #[arg(long, help = "subject")]
    pub sub: Option<String>,
    #[arg(long, help = "audience")]
    pub aud: Option<String>,
    #[arg(long, help = "jwt issuer")]
    pub iss: Option<String>,
    #[arg(long, default_value = "1d", help = "jwt expiration time", value_parser=parse_duration)]
    pub exp: u64,
    #[arg(long, default_value = "1d", help = "jwt nbf time", value_parser=parse_duration)]
    pub nbf: Option<u64>,
    #[arg(long, default_value_t = false, help = "generate jwt iat or not")]
    pub iat: bool,
}

#[derive(Debug, Parser)]
pub struct JwtVerifyOpts {
    // token and alg verified key
    #[arg(short, long, help = "jwt token", required = true, help = "jwt token")]
    pub token: String,
    #[arg(long, value_parser=parse_input_file, default_value="-", help = "key file path, or '-' for stdin")]
    pub key: String,

    #[arg(long, value_delimiter = ',', help = "required claims")]
    pub required_claims: Option<Vec<String>>,

    #[arg(long, value_delimiter = ',', help = "audiences")]
    pub aud: Option<Vec<String>>,
    #[arg(long, value_delimiter = ',', help = "issuers")]
    pub iss: Option<Vec<String>>,
    #[arg(long, help = "jwt subject")]
    pub sub: Option<String>,

    #[arg(long, help = "validate expiration time")]
    pub validate_exp: Option<bool>,
    #[arg(long, help = "validate nbf time")]
    pub validate_nbf: Option<bool>,
    #[arg(long, help = "validate audience")]
    pub validate_aud: Option<bool>,
    #[arg(
        long,
        default_value_t = false,
        help = "show token claims, if verified successfully"
    )]
    pub silent: bool,
    #[arg(long, default_value_t = false, help = "show self options")]
    pub show_self: bool,
}

impl CmdExcutor for JwtSignOpts {
    async fn execute(self) -> Result<()> {
        let key = get_content(&self.key)?;
        let iat = get_current_timestamp();
        let exp = iat + self.exp;
        let nbf = self.nbf.map(|nbf| iat + nbf);
        // claims init
        let mut claims = json!({"exp": exp});
        for (k, v) in [("sub", self.sub), ("aud", self.aud), ("iss", self.iss)] {
            if let Some(v) = v {
                claims[k] = serde_json::Value::String(v);
            }
        }
        if self.iat {
            claims["iat"] = serde_json::Value::Number(iat.into());
        }
        if let Some(nbf) = nbf {
            claims["nbf"] = serde_json::Value::Number(nbf.into());
        }
        // sign jwt
        let token = process_jwt_sign(self.alg, &claims, key)?;
        println!("{}", token);
        Ok(())
    }
}

impl CmdExcutor for JwtVerifyOpts {
    async fn execute(self) -> Result<()> {
        if self.show_self {
            println!("{:?}", self);
        }
        let key = get_content(&self.key)?;
        let mut validation = Validation::default();
        validation.required_spec_claims = self
            .required_claims
            .map_or(HashSet::from_iter(["exp".to_owned()]), HashSet::from_iter);

        validation.validate_aud = self.validate_aud.map_or(true, |v| v);
        validation.validate_exp = self.validate_exp.map_or(true, |v| v);
        validation.validate_nbf = self.validate_nbf.map_or(false, |v| v);
        validation.iss = self.iss.map(HashSet::from_iter);
        validation.aud = self.aud.map(HashSet::from_iter);
        validation.sub = self.sub;

        process_jwt_verify(&self.token, key, validation)
            .inspect(|token| {
                println!("✓ jwt token verified");
                if !self.silent {
                    println!("{:?}", token)
                }
            })
            .inspect_err(|e| {
                println!("⚠ Signature not verified, with Err: {}", e);
            })
            .map(|_| ())
    }
}

impl CmdExcutor for JwtOpts {
    async fn execute(self) -> Result<()> {
        self.subcmd.execute().await
    }
}
