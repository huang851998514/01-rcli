use std::{fs, io::Read, path::Path};

use crate::TextSignFormat;
use crate::{process_genpass, utils::read_data};
use anyhow::{Ok, Result};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};

pub trait TextSign {
    /// 文本签名
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

pub trait TextVerify {
    /// 文本验证签名
    fn verify(&self, reader: impl Read, signature: &[u8]) -> Result<bool>;
}

pub trait TextLoader {
    fn load(path: impl AsRef<Path>) -> Result<Self>
    where
        Self: Sized;
}

pub trait KeyGenerator {
    fn generate() -> Result<Vec<Vec<u8>>>;
}

pub struct Blake3 {
    key: [u8; 32],
}

impl TextSign for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        //TODO: 也可以一块一块的读取
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let data = blake3::keyed_hash(&self.key, &buf).as_bytes().to_vec();
        //println!("keyed_hash:{:?}", &data);
        Ok(data)
    }
}

impl TextVerify for Blake3 {
    fn verify(&self, mut reader: impl Read, signature: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let binding = blake3::keyed_hash(&self.key, &buf);
        let data = binding.as_bytes();
        Ok(data == signature)
    }
}

impl TextLoader for Blake3 {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl KeyGenerator for Blake3 {
    fn generate() -> Result<Vec<Vec<u8>>> {
        let key = process_genpass(32, false, false, false, false)?;
        Ok(vec![key.into_bytes()])
    }
}

impl Blake3 {
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = &key[..32];
        let key = key.try_into()?;
        Ok(Self::new(key))
    }
}

pub struct Ed25519Signer {
    key: SigningKey,
}

impl TextSign for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let signature = self.key.sign(&buf);
        Ok(signature.to_bytes().to_vec())
    }
}

impl TextLoader for Ed25519Signer {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl KeyGenerator for Ed25519Signer {
    fn generate() -> Result<Vec<Vec<u8>>> {
        let mut csprng = rand::rngs::OsRng;

        let sk = SigningKey::generate(&mut csprng);
        let pk = sk.verifying_key();
        let sk = sk.to_bytes().to_vec();
        let pk = pk.to_bytes().to_vec();
        Ok(vec![sk, pk])
    }
}

impl Ed25519Signer {
    pub fn new(key: SigningKey) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = SigningKey::from_bytes(key.try_into()?);
        Ok(Self::new(key))
    }
}

struct Ed25519Verifier {
    key: VerifyingKey,
}

impl TextVerify for Ed25519Verifier {
    fn verify(&self, mut reader: impl Read, signature: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = Signature::from_bytes(signature.try_into()?);
        let ret = self.key.verify(&buf, &sig).is_ok();
        Ok(ret)
    }
}

impl Ed25519Verifier {
    pub fn new(key: VerifyingKey) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = VerifyingKey::from_bytes(key.try_into()?)?;
        Ok(Self::new(key))
    }
}

impl TextLoader for Ed25519Verifier {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

pub fn process_text_sign(input: &str, key: &str, format: TextSignFormat) -> Result<String> {
    let mut reader = read_data(input)?;
    let signed = match format {
        TextSignFormat::Blake3 => {
            let sign = Blake3::load(key)?;
            sign.sign(&mut reader)?
        }
        TextSignFormat::Ed25519 => {
            let sign = Ed25519Signer::load(key)?;
            sign.sign(&mut reader)?
        }
    };
    let signed = URL_SAFE_NO_PAD.encode(signed);
    Ok(signed)
}

pub fn process_text_verify(
    input: &str,
    key: &str,
    signature: &str,
    format: TextSignFormat,
) -> Result<bool> {
    let mut reader = read_data(input)?;
    let signature = URL_SAFE_NO_PAD.decode(signature)?;
    //println!("signature:{:?}",signature);
    let verify = match format {
        TextSignFormat::Blake3 => {
            let verify = Blake3::load(key)?;
            //println!("verify:{:?}",verify.key);
            verify.verify(&mut reader, &signature)?
        }
        TextSignFormat::Ed25519 => {
            let verify = Ed25519Verifier::load(key)?;
            verify.verify(&mut reader, &signature)?
        }
    };
    Ok(verify)
}

pub fn process_text_generate(format: TextSignFormat) -> Result<Vec<Vec<u8>>> {
    match format {
        TextSignFormat::Blake3 => Blake3::generate(),
        TextSignFormat::Ed25519 => Ed25519Signer::generate(),
    }
}
