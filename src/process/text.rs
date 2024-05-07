use std::io::Read;

use anyhow::{Ok, Result};

trait TextSign {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

trait TextVerify {
    fn verify(&self, reader: impl Read, signature: &[u8]) -> Result<bool>;
}
#[allow(dead_code)]
struct Blake3 {
    key: [u8; 32],
}

impl TextSign for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        //TODO: 也可以一块一块的读取
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let data = blake3::hash(&buf).as_bytes().to_vec();
        Ok(data)
    }
}

impl TextVerify for Blake3 {
    fn verify(&self, mut reader: impl Read, signature: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let binding = blake3::hash(&buf);
        let data = binding.as_bytes();
        Ok(data == signature)
    }
}
