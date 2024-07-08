use crate::{get_reader, process_genpass, TextSignFormat};
use anyhow::{Ok, Result};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit},
    ChaCha20Poly1305, Nonce,
};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;
use std::{fs, io::Read, path::Path, vec};

pub trait TextSign {
    // &[u8] implements Read, so we can test with &[u8] instead of file
    // Sign the data from the reader and return the signature
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

pub trait TextVerify {
    fn verify(&self, reader: impl Read, signature: &[u8]) -> Result<bool>;
}

pub trait KeyLoader {
    fn load(path: impl AsRef<Path>) -> Result<Self>
    where
        Self: Sized;
}

pub trait KeyGenerator {
    fn generate() -> Result<Vec<Vec<u8>>>
    where
        Self: Sized;
}

pub trait TextEncrypt {
    fn encrypt(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

pub trait TextDecrypt {
    fn decrypt(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

pub struct Blake3 {
    key: [u8; 32],
}

pub struct Ed25519Signer {
    key: SigningKey,
}

pub struct Ed25519Verifier {
    key: VerifyingKey,
}

struct SecureChaCha20 {
    chacha20: ChaCha20Poly1305,
}

pub async fn process_text_sign(
    input: &str,
    private_key: &str,
    format: TextSignFormat,
) -> Result<String> {
    let mut reader = get_reader(input)?;

    let signed = match format {
        TextSignFormat::Blake3 => {
            let signer: Blake3 = Blake3::load(private_key)?;
            signer.sign(&mut reader)?
        }
        TextSignFormat::Ed25519 => {
            let signer = Ed25519Signer::load(private_key)?;
            signer.sign(&mut reader)?
        }
        _ => unimplemented!(),
    };
    let signed = URL_SAFE_NO_PAD.encode(signed);
    Ok(signed)
}

pub async fn process_text_verify(
    input: &str,
    public_key: &str,
    format: TextSignFormat,
    signature: &str,
) -> Result<bool> {
    let mut reader = get_reader(input)?;

    let signature = URL_SAFE_NO_PAD.decode(signature)?;
    let verified = match format {
        TextSignFormat::Blake3 => {
            let verifier = Blake3::load(public_key)?;
            verifier.verify(&mut reader, &signature)?
        }
        TextSignFormat::Ed25519 => {
            let verifier = Ed25519Verifier::load(public_key)?;
            verifier.verify(&mut reader, &signature)?
        }
        _ => unimplemented!(),
    };
    Ok(verified)
}

pub fn process_text_key_generate(format: TextSignFormat) -> Result<Vec<Vec<u8>>> {
    match format {
        TextSignFormat::Blake3 => Blake3::generate(),
        TextSignFormat::Ed25519 => Ed25519Signer::generate(),
        TextSignFormat::Chacha20 => SecureChaCha20::generate(),
    }
}

pub async fn process_text_encrypt(input: &str, key: &str) -> Result<String> {
    let mut reader: Box<dyn Read> = get_reader(input)?;
    let cipher: SecureChaCha20 = SecureChaCha20::load(key)?;
    let ciphertext = cipher.encrypt(&mut reader)?;

    let ciphertext = String::from_utf8(ciphertext)?;
    Ok(ciphertext)
}

pub async fn process_text_decrypt(input: &str, key: &str) -> Result<String> {
    let mut reader: Box<dyn Read> = get_reader(input)?;
    let cipher: SecureChaCha20 = SecureChaCha20::load(key)?;
    let plaintext: Vec<u8> = cipher.decrypt(&mut reader)?;

    let plaintext = String::from_utf8(plaintext)?;
    Ok(plaintext)
}

impl TextSign for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        // TODO: improve perf by reading in chunks
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let hash = blake3::keyed_hash(&self.key, &buf);
        Ok(hash.as_bytes().to_vec())
    }
}

impl TextSign for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sign: &Signature = &self.key.sign(&buf);
        Ok(sign.to_bytes().to_vec())
    }
}

impl TextVerify for Blake3 {
    fn verify(&self, mut reader: impl Read, signature: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let hash = blake3::keyed_hash(&self.key, &buf);
        let hash = hash.as_bytes().to_vec();
        Ok(hash == signature)
    }
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

impl KeyLoader for Blake3 {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl KeyLoader for Ed25519Signer {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key[..32].try_into()?)
    }
}

impl KeyLoader for SecureChaCha20 {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key[..32].try_into()?)
    }
}

impl KeyLoader for Ed25519Verifier {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key[..32].try_into()?)
    }
}

impl KeyGenerator for Blake3 {
    fn generate() -> Result<Vec<Vec<u8>>> {
        let key = process_genpass(32, true, true, true, true)?;
        Ok(vec![key.as_bytes().to_vec()])
    }
}

impl KeyGenerator for Ed25519Signer {
    fn generate() -> Result<Vec<Vec<u8>>> {
        let mut csprng = OsRng;
        let sk: SigningKey = SigningKey::generate(&mut csprng);
        let pk = sk.verifying_key().to_bytes().to_vec();
        let sk = sk.to_bytes().to_vec();
        Ok(vec![sk, pk])
    }
}

impl KeyGenerator for SecureChaCha20 {
    fn generate() -> Result<Vec<Vec<u8>>> {
        let key = ChaCha20Poly1305::generate_key(&mut OsRng).to_vec();
        Ok(vec![key])
    }
}

impl TextEncrypt for SecureChaCha20 {
    fn encrypt(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
        let ciphertext = &self.chacha20.encrypt(&nonce, buf.as_ref())?;

        let mut nonce_and_ciphertext = Vec::new();
        nonce_and_ciphertext.extend_from_slice(&nonce);
        nonce_and_ciphertext.extend(ciphertext);

        let nonce_and_ciphertext = URL_SAFE_NO_PAD
            .encode(nonce_and_ciphertext)
            .as_bytes()
            .to_vec();
        Ok(nonce_and_ciphertext)
    }
}

impl TextDecrypt for SecureChaCha20 {
    fn decrypt(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;

        let nonce_and_ciphertext = URL_SAFE_NO_PAD.decode(buf)?;
        let nonce = Nonce::from_slice(&nonce_and_ciphertext[..12]);
        let ciphertext = &nonce_and_ciphertext[12..];

        let plaintext = &self.chacha20.decrypt(nonce, ciphertext.as_ref())?;
        Ok(plaintext.to_vec())
    }
}

impl Blake3 {
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = &key[..32];
        let key = key.try_into()?;
        let signer = Blake3::new(key);
        Ok(signer)
    }
}

impl Ed25519Signer {
    pub fn new(key: SigningKey) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8; 32]) -> Result<Self> {
        let key = SigningKey::from_bytes(key);
        let signer = Ed25519Signer::new(key);
        Ok(signer)
    }
}

impl Ed25519Verifier {
    pub fn new(key: VerifyingKey) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8; 32]) -> Result<Self> {
        let key = VerifyingKey::from_bytes(key)?;
        let verifier = Ed25519Verifier::new(key);
        Ok(verifier)
    }
}

impl SecureChaCha20 {
    pub fn new(key: ChaCha20Poly1305) -> Self {
        Self { chacha20: key }
    }

    pub fn try_new(key: &[u8; 32]) -> Result<Self> {
        let key = ChaCha20Poly1305::new_from_slice(key)?;
        let cipher = SecureChaCha20::new(key);
        Ok(cipher)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blake3_sign_verify() -> Result<()> {
        let balke3 = Blake3::load("fixtures/blake3.txt")?;
        let data = b"hello world";
        let signature = balke3.sign(&mut &data[..]).unwrap();
        assert!(balke3.verify(&mut &data[..], &signature).unwrap());
        Ok(())
    }

    #[test]
    fn test_chacha20_encrypt_decrypt() -> Result<()> {
        let secure_chacha20 = SecureChaCha20::load("fixtures/chacha20.key")?;
        let mut data = "hello world".as_bytes();
        let ciphertext = secure_chacha20.encrypt(&mut data)?;
        let plaintext = secure_chacha20.decrypt(&mut &ciphertext[..])?;
        assert_eq!("hello world".as_bytes(), plaintext.as_slice());
        Ok(())
    }
}
