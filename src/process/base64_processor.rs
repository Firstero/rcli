use std::io::Read;

use crate::cli::Base64Format;
use base64::prelude::*;

// input 可以是 '-', 表示从 stdin 读取，或者是文件路径, 输出到 stdout, 返回一个 reader
fn get_reader(input: &str) -> anyhow::Result<Box<dyn Read>> {
    match input {
        "-" => Ok(Box::new(std::io::stdin())),
        path => Ok(Box::new(std::fs::File::open(path)?)),
    }
}

// base64 encoder,
pub fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader = get_reader(input)?;
    // 读取所有的数据
    let mut data = Vec::new();
    reader.read_to_end(&mut data)?;
    let encoded = match format {
        Base64Format::Standard => BASE64_STANDARD.encode(&data),
        Base64Format::UrlSafe => BASE64_URL_SAFE.encode(&data),
        Base64Format::NoPadding => BASE64_URL_SAFE_NO_PAD.encode(&data),
    };
    println!("{}", encoded);
    Ok(())
}

pub fn process_decode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader = get_reader(input)?;
    // 读取所有的数据
    let mut data = String::new();
    reader.read_to_string(&mut data)?;
    let data = data.trim_end();
    let decode = match format {
        Base64Format::Standard => BASE64_STANDARD.decode(data)?,
        Base64Format::UrlSafe => BASE64_URL_SAFE.decode(data)?,
        Base64Format::NoPadding => BASE64_URL_SAFE_NO_PAD.decode(data)?,
    };
    println!("{}", String::from_utf8(decode)?);
    Ok(())
}
