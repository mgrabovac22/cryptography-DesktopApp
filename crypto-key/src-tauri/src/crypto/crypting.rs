use std::fs;
use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use rsa::PublicKey;
use rand::{rngs::OsRng, RngCore};
use sha2::Sha256;
use rsa::Oaep;
use aes_gcm::aead::Aead;
use tauri::{AppHandle, Manager};

use super::keys::{load_secret_key, load_public_key, load_private_key};
use crate::logger::logger::write_log_entry;

pub fn symmetric_encrypt(app_handle: &AppHandle, input_path: String, output_path: String) -> Result<String, String> {
    write_log_entry(app_handle, &format!("Starting symmetric encryption for file: {}", input_path)).ok();

    let base_path = app_handle
        .path()
        .resolve("keys", tauri::path::BaseDirectory::AppData)
        .map_err(|e| {
            let msg = format!("Cannot determine keys directory: {}", e);
            write_log_entry(app_handle, &msg).ok();
            msg
        })?;

    let mut key_path = base_path.clone();
    key_path.push("secret_key.txt");

    let key = match load_secret_key(key_path.to_str().unwrap()) {
        Ok(k) => k,
        Err(e) => {
            let msg = format!("Failed to load secret key: {}", e);
            write_log_entry(app_handle, &msg).ok();
            return Err(msg);
        }
    };

    let plaintext = fs::read(&input_path)
        .map_err(|e| {
            let msg = format!("Error reading input file: {}", e);
            write_log_entry(app_handle, &msg).ok();
            msg
        })?;

    let cipher = Aes256Gcm::new(&key);
    let mut rng = OsRng;

    let mut nonce_bytes = [0u8; 12];
    rng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext_with_tag = cipher.encrypt(nonce, plaintext.as_ref())
        .map_err(|_| {
            let msg = "Symmetric encryption failed".to_string();
            write_log_entry(app_handle, &msg).ok();
            msg
        })?;

    let mut output_data = Vec::with_capacity(nonce.len() + ciphertext_with_tag.len());
    output_data.extend_from_slice(nonce.as_slice());
    output_data.extend(ciphertext_with_tag);

    fs::write(&output_path, output_data)
        .map_err(|e| {
            let msg = format!("Error writing encrypted output file: {}", e);
            write_log_entry(app_handle, &msg).ok();
            msg
        })?;

    let success_msg = format!("✅ Symmetric encryption completed successfully. Output: {}", output_path);
    write_log_entry(app_handle, &success_msg).ok();

    Ok(success_msg)
}

pub fn symmetric_decrypt(app_handle: &AppHandle, input_path: &str, output_path: &str) -> Result<String, String> {
    write_log_entry(app_handle, &format!("Starting symmetric decryption for file: {}", input_path)).ok();

    let base_path = app_handle
        .path()
        .resolve("keys", tauri::path::BaseDirectory::AppData)
        .map_err(|e| {
            let msg = format!("Cannot determine keys directory: {}", e);
            write_log_entry(app_handle, &msg).ok();
            msg
        })?;

    let mut key_path = base_path.clone();
    key_path.push("secret_key.txt");

    let key = match load_secret_key(key_path.to_str().unwrap()) {
        Ok(k) => k,
        Err(e) => {
            let msg = format!("Failed to load secret key: {}", e);
            write_log_entry(app_handle, &msg).ok();
            return Err(msg);
        }
    };

    let encrypted_data = fs::read(input_path)
        .map_err(|e| {
            let msg = format!("Error reading encrypted file: {}", e);
            write_log_entry(app_handle, &msg).ok();
            msg
        })?;

    const NONCE_LEN: usize = 12;
    if encrypted_data.len() < NONCE_LEN {
        let msg = "Encrypted file is too short or corrupted.".to_string();
        write_log_entry(app_handle, &msg).ok();
        return Err(msg);
    }

    let (nonce_slice, ciphertext_with_tag) = encrypted_data.split_at(NONCE_LEN);
    let nonce = Nonce::from_slice(nonce_slice);

    let cipher = Aes256Gcm::new(&key);

    let decrypted_data = cipher.decrypt(nonce, ciphertext_with_tag.as_ref())
        .map_err(|_| {
            let msg = "Symmetric decryption failed — invalid key or tampered data.".to_string();
            write_log_entry(app_handle, &msg).ok();
            msg
        })?;

    fs::write(output_path, decrypted_data)
        .map_err(|e| {
            let msg = format!("Error writing decrypted output: {}", e);
            write_log_entry(app_handle, &msg).ok();
            msg
        })?;

    let success_msg = format!("✅ Symmetric decryption completed successfully. Output: {}", output_path);
    write_log_entry(app_handle, &success_msg).ok();

    Ok(success_msg)
}

pub fn asymmetric_encrypt(app_handle: &AppHandle, input_path: &str, output_path: &str) -> Result<String, String> {
    write_log_entry(app_handle, &format!("Starting asymmetric encryption for file: {}", input_path)).ok();

    let app_data_dir = app_handle.path().app_data_dir()
        .map_err(|e| format!("Cannot determine keys directory: {}", e))?;

    let public_key_path = app_data_dir.join("keys").join("public_key.txt");
    let public_key_path_str = public_key_path.to_str().ok_or("Invalid key path string")?;

    let plaintext = fs::read(input_path)
        .map_err(|e| {
            let msg = format!("Error reading input file: {}", e);
            write_log_entry(app_handle, &msg).ok();
            msg
        })?;

    let public_key = load_public_key(public_key_path_str)?;

    let mut rng = OsRng;
    let padding = Oaep::new::<Sha256>();

    let ciphertext = public_key.encrypt(&mut rng, padding, &plaintext)
        .map_err(|e| {
            let msg = format!("Asymmetric encryption failed: {}", e);
            write_log_entry(app_handle, &msg).ok();
            msg
        })?;

    fs::write(output_path, ciphertext)
        .map_err(|e| {
            let msg = format!("Error writing encrypted file: {}", e);
            write_log_entry(app_handle, &msg).ok();
            msg
        })?;

    let success_msg = format!("✅ Asymmetric encryption completed successfully. Output: {}", output_path);
    write_log_entry(app_handle, &success_msg).ok();

    Ok(success_msg)
}

pub fn asymmetric_decrypt(app_handle: &AppHandle, input_path: &str, output_path: &str) -> Result<String, String> {
    write_log_entry(app_handle, &format!("Starting asymmetric decryption for file: {}", input_path)).ok();

    let app_data_dir = app_handle.path().app_data_dir()
        .map_err(|e| format!("Cannot determine keys directory: {}", e))?;

    let private_key_path = app_data_dir.join("keys").join("private_key.txt");
    let private_key_path_str = private_key_path.to_str().ok_or("Invalid key path string")?;

    let ciphertext = fs::read(input_path)
        .map_err(|e| {
            let msg = format!("Error reading encrypted input: {}", e);
            write_log_entry(app_handle, &msg).ok();
            msg
        })?;

    let private_key = load_private_key(private_key_path_str)?;
    let padding = Oaep::new::<Sha256>();

    let decrypted_data = private_key.decrypt(padding, ciphertext.as_ref())
        .map_err(|e| {
            let msg = format!("Asymmetric decryption failed: {}", e);
            write_log_entry(app_handle, &msg).ok();
            msg
        })?;

    fs::write(output_path, decrypted_data)
        .map_err(|e| {
            let msg = format!("Error writing decrypted output: {}", e);
            write_log_entry(app_handle, &msg).ok();
            msg
        })?;

    let success_msg = format!("✅ Asymmetric decryption completed successfully. Output: {}", output_path);
    write_log_entry(app_handle, &success_msg).ok();

    Ok(success_msg)
}
