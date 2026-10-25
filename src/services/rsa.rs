use rsa::{Oaep, RsaPrivateKey, RsaPublicKey};
use rsa::pkcs8::{DecodePrivateKey};
use rsa::pkcs1::{DecodeRsaPublicKey};
use rand::rngs::OsRng;
use sha2::Sha256;

// use std::io::{Error, ErrorKind};
// use std::path::Path;
use std::fs::{read_to_string};

pub fn create_rsa_key(size: usize) -> Result<RsaPrivateKey, String> {
    let key = RsaPrivateKey::new(&mut OsRng, size);
    if key.is_ok() {
        Ok(key.unwrap())
    } else {
        Err(String::from("an error occured while creating the private key"))
    }
}

pub fn get_public_key_from_private(private: RsaPrivateKey) -> RsaPublicKey {
    RsaPublicKey::from(&private)
}

pub fn get_private_from_file(file_path: &str) -> Result<RsaPrivateKey, String> {
    let pem = read_to_string(file_path);
    if pem.is_ok() {
        Ok(RsaPrivateKey::from_pkcs8_pem(pem.unwrap().as_str()).unwrap())
    } else {
        Err(String::from("didn't get the key, maybe the file path is incorrect"))
    }
    
}

pub fn get_public_from_file(file_path: &str) -> Result<RsaPublicKey, String> {
    let pem = read_to_string(file_path);
    if pem.is_ok() {
        Ok(RsaPublicKey::from_pkcs1_pem(pem.unwrap().as_str()).unwrap())    
    } else {
        Err(String::from("failed to read public key from file"))
    }
} 

pub fn encrypt(pubkey: RsaPublicKey, text: &str) -> Result<Vec<u8>, String>{
    let cipher = pubkey.encrypt(&mut OsRng, Oaep::new::<Sha256>(), text.as_bytes());
    if cipher.is_ok() {
        Ok(cipher.unwrap())
    } else {
        Err(String::from("an error occured while encrypting the secret"))
    }
}

pub fn decrypt(private: RsaPrivateKey, cipher_text: &[u8]) -> Result<Vec<u8>, String> {
    let text = private.decrypt(Oaep::new::<Sha256>(), cipher_text);
    if text.is_ok() {
        Ok(text.unwrap())
    } else {
        Err(String::from("an error occured while decrypting the cipher text"))
    }
}
