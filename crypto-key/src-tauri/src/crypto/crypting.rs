use std::fs;
use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use rsa::{PublicKey};
use rand::{rngs::OsRng, RngCore};
use sha2::Sha256;
use rsa::Oaep;
use super::keys::{load_secret_key, load_public_key, load_private_key};
use aes_gcm::aead::Aead;
use tauri::AppHandle;
use tauri::Manager;

pub fn symmetric_encrypt(app_handle: &AppHandle, input_path: String, output_path: String) -> Result<String, String> {
    let base_path = app_handle
        .path()
        .resolve("keys", tauri::path::BaseDirectory::AppData)
        .map_err(|e| format!("Cannot determine keys directory: {}", e))?;

    let mut key_path = base_path.clone();
    key_path.push("secret_key.txt");

    let key = load_secret_key(key_path.to_str().unwrap())?;

    let plaintext = fs::read(input_path)
        .map_err(|e| format!("Error reading input file: {}", e))?;

    let cipher = Aes256Gcm::new(&key);
    let mut rng = OsRng;

    let mut nonce_bytes = [0u8; 12];
    rng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext_with_tag = cipher.encrypt(nonce, plaintext.as_ref())
        .map_err(|_| "Symmetric encryption failed".to_string())?;

    let mut output_data = Vec::with_capacity(nonce.len() + ciphertext_with_tag.len());
    output_data.extend_from_slice(nonce.as_slice());
    output_data.extend(ciphertext_with_tag);

    fs::write(&output_path, output_data)
        .map_err(|e| format!("Error writing output file: {}", e))?;

    Ok(format!("Symmetric encryption completed. Data saved to: {}", output_path))
}

pub fn symmetric_decrypt(app_handle: &AppHandle, input_path: &str, output_path: &str) -> Result<String, String> {
    let base_path = app_handle
        .path()
        .resolve("keys", tauri::path::BaseDirectory::AppData)
        .map_err(|e| format!("Cannot determine keys directory: {}", e))?;

    let mut key_path = base_path.clone();
    key_path.push("secret_key.txt");

    let key = load_secret_key(key_path.to_str().unwrap())?;

    let encrypted_data = fs::read(input_path)
        .map_err(|e| format!("Error reading input file: {}", e))?;

    const NONCE_LEN: usize = 12;
    if encrypted_data.len() < NONCE_LEN {
        return Err("Encrypted file is too short or corrupted.".to_string());
    }

    let (nonce_slice, ciphertext_with_tag) = encrypted_data.split_at(NONCE_LEN);
    let nonce = Nonce::from_slice(nonce_slice);

    let cipher = Aes256Gcm::new(&key);

    let decrypted_data = cipher.decrypt(nonce, ciphertext_with_tag.as_ref())
        .map_err(|_| "Symmetric decryption failed. Key/data is incorrect or file is tampered.".to_string())?;

    fs::write(output_path, decrypted_data)
        .map_err(|e| format!("Error writing output file: {}", e))?;

    Ok(format!("Symmetric decryption completed. Data saved to: {}", output_path))
}

pub fn asymmetric_encrypt(app_handle: &tauri::AppHandle, input_path: &str, output_path: &str) -> Result<String, String> {
    let app_data_dir = app_handle.path().app_data_dir()
        .map_err(|e| format!("Cannot determine keys directory: {}", e))?;

    let public_key_path = app_data_dir
        .join("keys")
        .join("public_key.txt");

    let public_key_path_str = public_key_path
        .to_str()
        .ok_or_else(|| "Invalid key path string".to_string())?;
        
    let plaintext = std::fs::read(input_path)
        .map_err(|e| format!("Error reading input file: {}", e))?;
    
    let public_key = load_public_key(public_key_path_str)?;

    let mut rng = OsRng;
    let padding = Oaep::new::<Sha256>();

    let ciphertext = public_key.encrypt(&mut rng, padding, &plaintext)
        .map_err(|e| format!("Encryption failed: {}", e))?;

    std::fs::write(output_path, ciphertext)
        .map_err(|e| format!("Error writing output file: {}", e))?;

    Ok(format!("Asymmetric encryption done. Saved to {}", output_path))
}


pub fn asymmetric_decrypt(app_handle: &tauri::AppHandle, input_path: &str, output_path: &str) -> Result<String, String> {
    let app_data_dir = app_handle.path().app_data_dir()
        .map_err(|e| format!("Cannot determine keys directory: {}", e))?;

    let private_key_path = app_data_dir
        .join("keys")
        .join("private_key.txt");

    let private_key_path_str = private_key_path
        .to_str()
        .ok_or_else(|| "Invalid key path string".to_string())?;
        
    let ciphertext = fs::read(input_path)
        .map_err(|e| format!("Error reading input file: {}", e))?;
    
    let private_key = load_private_key(private_key_path_str)?;

    let padding = Oaep::new::<Sha256>();

    let decrypted_data = private_key.decrypt(
        padding,
        ciphertext.as_ref()
    )
    .map_err(|e| format!("Asymmetric decryption failed. Private key/data is incorrect: {}", e))?;

    fs::write(output_path, decrypted_data)
        .map_err(|e| format!("Error writing output file: {}", e))?;

    Ok(format!("Asymmetric decryption completed. Data saved to: {}", output_path))
}
