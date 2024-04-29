use crate::TextSignFormat;
use anyhow::{anyhow, Result};
use chacha20poly1305::aead::generic_array::GenericArray;
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305, Nonce,
};
use ed25519::signature::{Signer, Verifier};
use ed25519_dalek::{Signature, SigningKey, VerifyingKey};
use std::collections::HashMap;
use std::io::Read;
trait TextSign {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

trait TextVerify {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<bool>;
}

trait KeyGenerator {
    fn generate() -> Result<HashMap<&'static str, Vec<u8>>>;
}

trait Encrypt {
    fn encrypt(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

trait Decrypt {
    fn decrypt(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
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

impl TextSign for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        // TODO: improve perf by reading in chunks
        let mut data = Vec::new();
        reader.read_to_end(&mut data)?;
        let hash = blake3::keyed_hash(&self.key, &data);
        Ok(hash.as_bytes().to_vec())
    }
}

impl KeyGenerator for Blake3 {
    fn generate() -> Result<HashMap<&'static str, Vec<u8>>> {
        let mut csprng = OsRng;
        let sk: SigningKey = SigningKey::generate(&mut csprng);
        let pk: VerifyingKey = (&sk).into();
        let mut map = HashMap::new();
        map.insert("ed25519.sk", sk.to_bytes().to_vec());
        map.insert("ed25519.pk", pk.to_bytes().to_vec());
        Ok(map)
    }
}

impl TextVerify for Blake3 {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<bool> {
        let mut data = Vec::new();
        reader.read_to_end(&mut data)?;
        let hash = blake3::keyed_hash(&self.key, &data);
        Ok(hash.as_bytes() == sig)
    }
}

struct Ed25519Signer {
    key: SigningKey,
}

impl Ed25519Signer {
    pub fn new(key: SigningKey) -> Self {
        Self { key }
    }

    pub fn try_new(key: impl AsRef<[u8]>) -> Result<Self> {
        let key = key.as_ref();
        let key = SigningKey::from_bytes(key.try_into()?);
        Ok(Self::new(key))
    }
}

impl TextSign for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut data = Vec::new();
        reader.read_to_end(&mut data)?;
        let sig = self.key.sign(&data);
        Ok(sig.to_bytes().to_vec())
    }
}

impl KeyGenerator for Ed25519Signer {
    fn generate() -> Result<HashMap<&'static str, Vec<u8>>> {
        let mut rng = rand::rngs::OsRng;
        let sk = SigningKey::generate(&mut rng);
        let pk = VerifyingKey::from(&sk);
        let mut map = HashMap::new();
        map.insert("ed25519.sk", sk.to_bytes().to_vec());
        map.insert("ed25519.pk", pk.to_bytes().to_vec());
        Ok(map)
    }
}

struct Ed25519Verifier {
    key: VerifyingKey,
}

impl Ed25519Verifier {
    pub fn new(key: VerifyingKey) -> Self {
        Self { key }
    }

    pub fn try_new(key: impl AsRef<[u8]>) -> Result<Self> {
        let key = key.as_ref();
        VerifyingKey::from_bytes(key.try_into()?)
            .map_or_else(|_| Err(anyhow!("invalid key")), |key| Ok(Self::new(key)))
    }
}

impl TextVerify for Ed25519Verifier {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<bool> {
        let mut data = Vec::new();
        reader.read_to_end(&mut data)?;
        let sig = Signature::from_slice(sig).map_err(|_| anyhow!("invalid signature"))?;
        let ret = self.key.verify(&data, &sig).is_ok();
        Ok(ret)
    }
}

struct ChaCha20Poly1305cryptor {
    nonce: Nonce,
    cipher: ChaCha20Poly1305,
}

impl ChaCha20Poly1305cryptor {
    pub fn new(key: &[u8], nonce: Nonce) -> Self {
        Self {
            cipher: ChaCha20Poly1305::new(key.into()),
            nonce,
        }
    }

    pub fn try_new(key: impl AsRef<[u8]>, nonce: impl AsRef<[u8]>) -> Result<Self> {
        let key = key.as_ref();
        let nonce = Nonce::from_slice(nonce.as_ref());
        Ok(Self::new(key, *nonce))
    }
}

impl KeyGenerator for ChaCha20Poly1305cryptor {
    fn generate() -> Result<HashMap<&'static str, Vec<u8>>> {
        let key = ChaCha20Poly1305::generate_key(&mut OsRng);
        let nonce: GenericArray<u8, _> = ChaCha20Poly1305::generate_nonce(&mut OsRng);
        let mut map = HashMap::new();
        map.insert("chacha20poly1305.key", key.to_vec());
        map.insert("chacha20poly1305.nonce", nonce.to_vec());
        Ok(map)
    }
}

impl Encrypt for ChaCha20Poly1305cryptor {
    fn encrypt(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut data = Vec::new();
        reader.read_to_end(&mut data)?;
        let ciphertext = self
            .cipher
            .encrypt(&self.nonce, data.as_ref())
            .map_err(|v| anyhow!(format!("encrypt error, {}", v)))?;
        Ok(ciphertext)
    }
}

impl Decrypt for ChaCha20Poly1305cryptor {
    fn decrypt(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut data = Vec::new();
        reader.read_to_end(&mut data)?;
        let plaintext = self
            .cipher
            .decrypt(&self.nonce, data.as_ref())
            .map_err(|v| anyhow!(format!("decrypt error, {}", v)))?;
        Ok(plaintext)
    }
}

/// 根据format 调用不同的signer, 依据种子key对输入文本进行签名
pub fn process_sign(reader: &mut dyn Read, key: &[u8], format: TextSignFormat) -> Result<Vec<u8>> {
    let signer: Box<dyn TextSign> = match format {
        TextSignFormat::Blake3 => Box::new(Blake3::try_new(key)?),
        TextSignFormat::Ed25519 => Box::new(Ed25519Signer::try_new(key)?),
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
    // 修改下列错误
    let verifier: Box<dyn TextVerify> = match format {
        TextSignFormat::Blake3 => Box::new(Blake3::try_new(key)?),
        TextSignFormat::Ed25519 => Box::new(Ed25519Verifier::try_new(key)?),
    };
    verifier.verify(reader, sig)
}

pub fn process_generate(format: TextSignFormat) -> Result<HashMap<&'static str, Vec<u8>>> {
    match format {
        TextSignFormat::Blake3 => Blake3::generate(),
        TextSignFormat::Ed25519 => Ed25519Signer::generate(),
    }
}

// 加密
pub fn process_encrypt(reader: &mut dyn Read, key: &[u8], nonce: &[u8]) -> Result<Vec<u8>> {
    let encrpytor = ChaCha20Poly1305cryptor::try_new(key, nonce)?;
    encrpytor.encrypt(reader)
}

// 解密
pub fn process_decrypt(reader: &mut dyn Read, key: &[u8], nonce: &[u8]) -> Result<Vec<u8>> {
    let decrypter = ChaCha20Poly1305cryptor::try_new(key, nonce)?;
    decrypter.decrypt(reader)
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

    // 使用 fiturex/chacha20poly1305.key 和 fiturex/chacha20poly1305.nonce 测试 chacha20poly1305Encryptor
    #[test]
    fn test_process_encrypt() -> Result<()> {
        let key: &[u8] = include_bytes!("../../fixtures/chacha20poly1305.key");
        let nonce: &[u8] = include_bytes!("../../fixtures/chacha20poly1305.nonce");
        let encrypted = process_encrypt(&mut "hello,world!".as_bytes(), key, nonce)?;
        let decrypted = process_decrypt(&mut encrypted.as_slice(), key, nonce)?;
        assert_eq!("hello,world!".as_bytes(), decrypted);
        Ok(())
    }
}
