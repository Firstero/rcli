use crate::{cli::Base64Format, utils::get_content};
use anyhow::Result;
use base64::prelude::*;

// base64 encoder,
pub fn process_encode(input: &str, format: Base64Format) -> Result<String> {
    let data = get_content(input)?;
    let encoded = match format {
        Base64Format::Standard => BASE64_STANDARD.encode(&data),
        Base64Format::UrlSafe => BASE64_URL_SAFE.encode(&data),
        Base64Format::NoPadding => BASE64_URL_SAFE_NO_PAD.encode(&data),
    };
    Ok(encoded)
}

pub fn process_decode(input: &str, format: Base64Format) -> Result<String> {
    let data = get_content(input)?;
    let decode = match format {
        Base64Format::Standard => BASE64_STANDARD.decode(data)?,
        Base64Format::UrlSafe => BASE64_URL_SAFE.decode(data)?,
        Base64Format::NoPadding => BASE64_URL_SAFE_NO_PAD.decode(data)?,
    };
    Ok(String::from_utf8(decode)?)
}
