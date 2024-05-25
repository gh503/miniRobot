use ring::aead::{self, Aad, LessSafeKey, Nonce, UnboundKey};
use ring::rand::{SecureRandom, SystemRandom};
use ring::hmac;
use std::error::Error;
use base64::{engine::general_purpose, Engine as _};

#[derive(PartialEq)]
pub enum EncryptionAlgorithm {
    AES256GCM,
    ChaCha20,
}

impl EncryptionAlgorithm {
    pub fn tag_len(&self) -> usize {
        match self {
            EncryptionAlgorithm::AES256GCM => aead::AES_256_GCM.tag_len(),
            EncryptionAlgorithm::ChaCha20 => aead::CHACHA20_POLY1305.tag_len(),
        }
    }

    pub fn zero_array(&self) -> Vec<u8> {
        vec![0; self.tag_len()]
    }
}

pub struct Encryption {
    key: Vec<u8>,
    auth_password: String,
    algorithm: EncryptionAlgorithm,
}

impl Encryption {
    pub fn new(key: String, auth_password: String, algorithm: EncryptionAlgorithm) -> Result<Self, Box<dyn Error>> {
        // Ensure the key length is appropriate for the algorithm
        if algorithm == EncryptionAlgorithm::AES256GCM && key.len() != 32 {
            return Err("AES key length must be 32 bytes (256 bits)".into());
        }
        if algorithm == EncryptionAlgorithm::ChaCha20 && key.len() != 32 {
            return Err("ChaCha20 key length must be 32 bytes (256 bits)".into());
        }

        Ok(Encryption {
            key: key.into_bytes(),
            auth_password,
            algorithm,
        })
    }

    pub fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut in_out = data.to_vec();
        in_out.extend_from_slice(&self.algorithm.zero_array()); // Use zero_array from the algorithm
        let unbound_key = match self.algorithm {
            EncryptionAlgorithm::AES256GCM => UnboundKey::new(&aead::AES_256_GCM, &self.key)?,
            EncryptionAlgorithm::ChaCha20 => UnboundKey::new(&aead::CHACHA20_POLY1305, &self.key)?,
        };
        let key = LessSafeKey::new(unbound_key);
        let nonce = self.generate_nonce()?;
        key.seal_in_place_append_tag(Nonce::assume_unique_for_key(nonce), Aad::empty(), &mut in_out)?;
        Ok([nonce.as_ref(), in_out.as_slice()].concat())
    }

    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        let nonce_size = match self.algorithm {
            EncryptionAlgorithm::AES256GCM => aead::AES_256_GCM.nonce_len(),
            EncryptionAlgorithm::ChaCha20 => aead::CHACHA20_POLY1305.nonce_len(),
        };
        let (nonce, ciphertext) = data.split_at(nonce_size);
        let unbound_key = match self.algorithm {
            EncryptionAlgorithm::AES256GCM => UnboundKey::new(&aead::AES_256_GCM, &self.key)?,
            EncryptionAlgorithm::ChaCha20 => UnboundKey::new(&aead::CHACHA20_POLY1305, &self.key)?,
        };
        let key = LessSafeKey::new(unbound_key);
        let mut in_out = ciphertext.to_vec();
        key.open_in_place(Nonce::try_assume_unique_for_key(nonce)?, Aad::empty(), &mut in_out)?;
        let tag_len = self.algorithm.tag_len(); // Calculate tag_len
        in_out.truncate(in_out.len() - tag_len);
        Ok(in_out)
    }

    pub fn generate_nonce(&self) -> Result<[u8; 12], Box<dyn Error>> {
        let mut nonce = [0; 12];
        SystemRandom::new().fill(&mut nonce)?;
        Ok(nonce)
    }

    pub fn sign(&self, data: &str) -> String {
        let key = hmac::Key::new(hmac::HMAC_SHA256, self.auth_password.as_bytes());
        let tag = hmac::sign(&key, data.as_bytes());
        general_purpose::STANDARD.encode(tag.as_ref())
    }

    pub fn verify(&self, data: &str, signature: &str) -> bool {
        let key = hmac::Key::new(hmac::HMAC_SHA256, self.auth_password.as_bytes());
        let signature_bytes = match general_purpose::STANDARD.decode(signature) {
            Ok(bytes) => bytes,
            Err(_) => return false,
        };
        hmac::verify(&key, data.as_bytes(), &signature_bytes).is_ok()
    }
}