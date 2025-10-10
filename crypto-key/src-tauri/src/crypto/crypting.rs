use std::fs;
use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use rsa::{PublicKey, RsaPrivateKey, Oaep};
use rsa::PaddingScheme;
use rand::{rngs::OsRng, RngCore};
use sha2::Sha256;
use super::keys::{load_secret_key, load_public_key, load_private_key};
use aes_gcm::aead::Aead;


pub fn symmetric_encrypt(input_path: &str, output_path: &str) -> Result<String, String> {
    let plaintext = fs::read(input_path)
        .map_err(|e| format!("Error reading input file: {}", e))?;
    let key = load_secret_key("./keys/secret_key.txt")?;
    
    let cipher = Aes256Gcm::new(&key);
    let mut rng = OsRng;
    
    let mut nonce_bytes = [0u8; 12];
    rng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext_with_tag = cipher.encrypt(
        nonce, 
        plaintext.as_ref()
    )
    .map_err(|_| "Symmetric encryption failed (Internal error)".to_string())?;

    let mut output_data = Vec::with_capacity(nonce.len() + ciphertext_with_tag.len());
    output_data.extend_from_slice(nonce.as_slice());
    output_data.extend(ciphertext_with_tag);

    fs::write(output_path, output_data)
        .map_err(|e| format!("Error writing output file: {}", e))?;

    Ok(format!("Symmetric encryption completed. Data saved to: {}", output_path))
}

pub fn symmetric_decrypt(input_path: &str, output_path: &str) -> Result<String, String> {
    let encrypted_data = fs::read(input_path)
        .map_err(|e| format!("Error reading input file: {}", e))?;
    let key = load_secret_key("./keys/secret_key.txt")?;

    const NONCE_LEN: usize = 12;
    if encrypted_data.len() < NONCE_LEN {
        return Err("Encrypted file is too short/corrupted.".to_string());
    }
    
    let (nonce_slice, ciphertext_with_tag) = encrypted_data.split_at(NONCE_LEN);
    let nonce = Nonce::from_slice(nonce_slice);

    let cipher = Aes256Gcm::new(&key);

    let decrypted_data = cipher.decrypt(nonce, ciphertext_with_tag.as_ref())
        .map_err(|_| "Symmetric decryption FAILED. Key/data is incorrect or file is tampered.".to_string())?;

    fs::write(output_path, decrypted_data)
        .map_err(|e| format!("Error writing output file: {}", e))?;

    Ok(format!("Symmetric decryption completed. Data saved to: {}", output_path))
}

pub fn asymmetric_encrypt(input_path: &str, output_path: &str) -> Result<String, String> {
    let plaintext = std::fs::read(input_path)
        .map_err(|e| format!("Error reading input file: {}", e))?;
    let public_key = load_public_key("./keys/public_key.txt")?;

    let mut rng = OsRng;
    let padding = rsa::Oaep::new::<Sha256>();

    let ciphertext = public_key.encrypt(&mut rng, padding, &plaintext)
        .map_err(|e| format!("Encryption failed: {}", e))?;

    std::fs::write(output_path, ciphertext)
        .map_err(|e| format!("Error writing output file: {}", e))?;

    Ok(format!("Asymmetric encryption done. Saved to {}", output_path))
}

pub fn asymmetric_decrypt(input_path: &str, output_path: &str) -> Result<String, String> {
    let ciphertext = fs::read(input_path)
        .map_err(|e| format!("Error reading input file: {}", e))?;
    let private_key = load_private_key("./keys/private_key.txt")?;

    let decrypted_data = private_key.decrypt(
        rsa::Oaep::new::<Sha256>(),
        ciphertext.as_ref()
    )
    .map_err(|e| format!("Asymmetric decryption failed. Private key/data is incorrect: {}", e))?;

    fs::write(output_path, decrypted_data)
        .map_err(|e| format!("Error writing output file: {}", e))?;

    Ok(format!("Asymmetric decryption completed. Data saved to: {}", output_path))
}
