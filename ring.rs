use ring::aead::{self, BoundKey, Nonce, NonceSequence, Aad};
use ring::rand::SystemRandom;
use ring::digest;

fn main() {
    // Generate a random key
    let key = generate_random_key();
    
    // Encrypt and decrypt a message
    let plaintext = b"Hello, Rust!";
    let ciphertext = encrypt(plaintext, &key);
    let decrypted_text = decrypt(&ciphertext, &key);

    println!("Plaintext: {:?}", plaintext);
    println!("Ciphertext: {:?}", ciphertext);
    println!("Decrypted Text: {:?}", decrypted_text);
}

fn generate_random_key() -> [u8; 32] {
    let rng = SystemRandom::new();
    let mut key = [0u8; 32];
    rng.fill(&mut key).expect("Failed to generate random key");
    key
}

fn encrypt(plaintext: &[u8], key: &[u8; 32]) -> Vec<u8> {
    let nonce = generate_nonce();
    let sealing_key = aead::SealingKey::new(aead::AES_256_GCM, key)
        .expect("Failed to create sealing key");

    let mut ciphertext = Vec::with_capacity(plaintext.len() + sealing_key.algorithm().tag_len());
    let mut in_out = aead::Buffer::new(plaintext);

    sealing_key
        .seal_in_place_append_tag(nonce, Aad::empty(), &mut in_out)
        .expect("Encryption failed");

    ciphertext.extend_from_slice(in_out.as_ref());
    ciphertext
}

fn decrypt(ciphertext: &[u8], key: &[u8; 32]) -> Vec<u8> {
    let nonce = generate_nonce();
    let opening_key = aead::OpeningKey::new(aead::AES_256_GCM, key)
        .expect("Failed to create opening key");

    let mut decrypted_text = Vec::with_capacity(ciphertext.len() - opening_key.algorithm().tag_len());
    let mut in_out = aead::Buffer::new(ciphertext);

    opening_key
        .open_in_place(nonce, Aad::empty(), &mut in_out)
        .expect("Decryption failed");

    decrypted_text.extend_from_slice(in_out.as_ref());
    decrypted_text
}

fn generate_nonce() -> Nonce {
    let rng = SystemRandom::new();
    let mut nonce = Nonce::default();
    rng.fill(nonce.as_mut()).expect("Failed to generate nonce");
    nonce
}
