use std::{fs, io::Read};

use crate::{get_reader, TextSignFormat};
use anyhow::Result;

trait TextSign {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

trait TextVerify {
    fn verify(&self, reader: impl Read, sig: &[u8]) -> Result<bool>;
}

struct Blake3 {
    key: [u8; 32],
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
pub fn process_sign(input: &str, key: &str, format: TextSignFormat) -> Result<Vec<u8>> {
    let mut reader = get_reader(input)?;
    match format {
        TextSignFormat::Blake3 => {
            let key = fs::read(key)?;
            let signer = Blake3 {
                key: key.try_into().unwrap(),
            };
            signer.sign(&mut reader)
        }
        TextSignFormat::Ed25519 => {
            todo!()
        }
    }
}

pub fn process_verify(input: &str, key: &str, sig: &str, format: TextSignFormat) -> Result<bool> {
    let mut reader = get_reader(input)?;
    // 读取所有的数据
    match format {
        TextSignFormat::Blake3 => {
            let key = fs::read(key)?;
            let verifier = Blake3 {
                key: key.try_into().unwrap(),
            };
            verifier.verify(&mut reader, sig.as_bytes())
        }
        TextSignFormat::Ed25519 => {
            todo!()
        }
    }
}
