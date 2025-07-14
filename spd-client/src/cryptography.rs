use aes_siv::{Aes256Siv, Key};

pub fn encrypt(key: &[u8; 64], plaintext: &[u8]) -> Vec<u8> {
    let cipher = Aes256Siv::new(Key::from_slice(key));
    cipher.encrypt(&[], plaintext).unwrap()
}

pub fn decrypt(key: &[u8; 64], ciphertext: &[u8]) -> Vec<u8> {
    let cipher = Aes256Siv::new(Key::from_slice(key));
    cipher.decrypt(&[], ciphertext).unwrap()
}