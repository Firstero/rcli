use crate::TextSignFormat;
use anyhow::anyhow;
use anyhow::Result;
use std::io::Read;

trait TextSign {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

trait TextVerify {
    fn verify(&self, reader: impl Read, sig: &[u8]) -> Result<bool>;
}

struct Blake3 {
    key: [u8; 32],
}

impl Blake3 {
    // 实现 new 方法
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    pub fn try_new(key: impl AsRef<[u8]>) -> Result<Self> {
        let key = key.as_ref();
        let key = key
            .try_into()
            .map_err(|_| anyhow!("key must be 32 bytes"))?;
        Ok(Self::new(key))
    }
}

// struct Ed25519Signer {
//     key: [u8; 32],
// }

// struct Ed25519Verifier {
//     key: [u8; 32],
// }

impl TextSign for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        // TODO: improve perf by reading in chunks
        let mut data = Vec::new();
        reader.read_to_end(&mut data)?;
        let hash = blake3::keyed_hash(&self.key, &data);
        Ok(hash.as_bytes().to_vec())
    }
}

impl TextVerify for Blake3 {
    fn verify(&self, mut reader: impl Read, sig: &[u8]) -> Result<bool> {
        let mut data = Vec::new();
        reader.read_to_end(&mut data)?;
        let hash = blake3::keyed_hash(&self.key, &data);
        Ok(hash.as_bytes() == sig)
    }
}

/// 根据format 调用不同的signer, 依据种子key对输入文本进行签名
pub fn process_sign(reader: &mut dyn Read, key: &[u8], format: TextSignFormat) -> Result<Vec<u8>> {
    let signer = match format {
        TextSignFormat::Blake3 => Blake3::try_new(key)?,
        TextSignFormat::Ed25519 => todo!(),
    };
    signer.sign(reader)
}

pub fn process_verify(
    reader: &mut dyn Read,
    key: &[u8],
    sig: &[u8],
    format: TextSignFormat,
) -> Result<bool> {
    // 读取所有的数据
    let verifier = match format {
        TextSignFormat::Blake3 => Blake3::try_new(key)?,
        TextSignFormat::Ed25519 => todo!(),
    };
    verifier.verify(reader, sig)
}

// 生成测试用例
#[cfg(test)]
mod tests {
    use super::*;
    use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
    const KEY: &[u8] = b"2PVnPNxWEbfdPuLMMmjbwBL5e6B1LFBD";

    #[test]
    fn test_process_sign() -> Result<()> {
        let sig = URL_SAFE_NO_PAD.decode(b"EEkM_0sUgvngYIEG7ZGvQs0dTt3HF13pfVisK1aD6lg")?;
        assert_eq!(
            sig,
            process_sign(&mut "hello,world!".as_bytes(), KEY, TextSignFormat::Blake3)?
        );
        Ok(())
    }

    #[test]
    fn test_process_verify() -> Result<()> {
        let sig = URL_SAFE_NO_PAD.decode(b"EEkM_0sUgvngYIEG7ZGvQs0dTt3HF13pfVisK1aD6lg")?;
        let ret = process_verify(
            &mut "hello,world!".as_bytes(),
            KEY,
            &sig,
            TextSignFormat::Blake3,
        );
        assert!(ret.is_ok());
        Ok(())
    }
}
