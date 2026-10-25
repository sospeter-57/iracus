use argon2::Argon2;
use chacha20poly1305::{
    aead::{rand_core::RngCore, Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305, Key, Nonce,
};
use hkdf::Hkdf;
use sha2::Sha256;
// use base64::{engine::general_purpose::STANDARD, Engine};

// The key is computed from: password (Argon2id) + extra secret (HKDF).
// Both must be correct, and the salt must be the one used at encryption time.
fn derive_key(password: &[u8], extra_secret: &[u8], salt: &[u8; 16]) -> [u8; 32] {
    // step 1: slow, memory-hard stretching of the password
    let mut stretched = [0u8; 32];
    Argon2::default()
        .hash_password_into(password, salt, &mut stretched)
        .unwrap();

    // step 2: mix with the other secret into the final key
    let mut input = Vec::new();
    input.extend_from_slice(&stretched);
    input.extend_from_slice(extra_secret);

    let hk = Hkdf::<Sha256>::new(None, &input);
    let mut key = [0u8; 32];
    hk.expand(b"undecided", &mut key).unwrap();
    key
}

// output layout: [16-byte salt][12-byte nonce][ciphertext + 16-byte tag]
pub fn encrypt(password: &[u8], extra_secret: &[u8], plaintext: &[u8]) -> Vec<u8> {
    let mut salt = [0u8; 16];
    OsRng.fill_bytes(&mut salt);

    let key = derive_key(password, extra_secret, &salt);
    let cipher = ChaCha20Poly1305::new(Key::from_slice(&key));
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
    let ciphertext = cipher.encrypt(&nonce, plaintext).expect("encryption failed");

    let mut out = Vec::new();
    out.extend_from_slice(&salt);
    out.extend_from_slice(&nonce);
    out.extend_from_slice(&ciphertext);
    out
}

pub fn decrypt(password: &[u8], extra_secret: &[u8], data: &[u8]) -> Result<Vec<u8>, String> {
    if data.len() < 16 + 12 + 16 {
        return Err("data too short".to_string());
    }
    let (salt_bytes, rest) = data.split_at(16);
    let (nonce_bytes, ciphertext) = rest.split_at(12);

    let mut salt = [0u8; 16];
    salt.copy_from_slice(salt_bytes);

    let key = derive_key(password, extra_secret, &salt);
    let cipher = ChaCha20Poly1305::new(Key::from_slice(&key));
    cipher
        .decrypt(Nonce::from_slice(nonce_bytes), ciphertext)
        .map_err(|_| "wrong key material or tampered data".to_string())
}
