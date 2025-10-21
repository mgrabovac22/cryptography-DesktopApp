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

fn format_log(event: &str, details: &[(&str, &str)]) -> String {
    let details_str = details
        .iter()
        .map(|(k, v)| format!("{}: {}", k, v))
        .collect::<Vec<String>>()
        .join("; ");
    format!("EVENT: {}; {}", event, details_str)
}

pub fn symmetric_encrypt(app_handle: &AppHandle, input_path: String, output_path: String) -> Result<String, String> {
    write_log_entry(app_handle, &format_log("Symmetric Encrypt Start", &[("File", &input_path)])).ok();

    let base_path = app_handle
        .path()
        .resolve("keys", tauri::path::BaseDirectory::AppData)
        .map_err(|e| {
            let msg = format_log("Keys Dir Error", &[("Context", "Symmetric Encrypt"), ("Error", &e.to_string())]);
            write_log_entry(app_handle, &msg).ok();
            msg
        })?;

    let mut key_path = base_path.clone();
    key_path.push("secret_key.txt");
    let key_path_str = key_path.to_str().unwrap_or("Unknown path");

    let key = match load_secret_key(key_path_str) {
        Ok(k) => k,
        Err(e) => {
            let msg = format_log("Secret Key Load Failed", &[("Context", "Symmetric Encrypt"), ("Key Path", key_path_str), ("Error", &e)]);
            write_log_entry(app_handle, &msg).ok();
            return Err(msg);
        }
    };

    let plaintext = fs::read(&input_path)
        .map_err(|e| {
            let msg = format_log("File Read Failed", &[("Path", &input_path), ("Error", &e.to_string())]);
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
            let msg = format_log("Symmetric Encrypt Failed", &[("Reason", "Cipher Error")]);
            write_log_entry(app_handle, &msg).ok();
            msg
        })?;

    let mut output_data = Vec::with_capacity(nonce.len() + ciphertext_with_tag.len());
    output_data.extend_from_slice(nonce.as_slice());
    output_data.extend(ciphertext_with_tag);

    fs::write(&output_path, output_data)
        .map_err(|e| {
            let msg = format_log("File Write Failed", &[("Path", &output_path), ("Error", &e.to_string())]);
            write_log_entry(app_handle, &msg).ok();
            msg
        })?;

    let success_msg = format!("✅ Symmetric encryption completed successfully.");
    write_log_entry(app_handle, &format_log("Symmetric Encrypt Success", &[("Output Path", &output_path)])).ok();

    Ok(success_msg)
}

pub fn symmetric_decrypt(app_handle: &AppHandle, input_path: &str, output_path: &str) -> Result<String, String> {
    write_log_entry(app_handle, &format_log("Symmetric Decrypt Start", &[("File", input_path)])).ok();

    let base_path = app_handle
        .path()
        .resolve("keys", tauri::path::BaseDirectory::AppData)
        .map_err(|e| {
            let msg = format_log("Keys Dir Error", &[("Context", "Symmetric Decrypt"), ("Error", &e.to_string())]);
            write_log_entry(app_handle, &msg).ok();
            msg
        })?;

    let mut key_path = base_path.clone();
    key_path.push("secret_key.txt");
    let key_path_str = key_path.to_str().unwrap_or("Unknown path");

    let key = match load_secret_key(key_path_str) {
        Ok(k) => k,
        Err(e) => {
            let msg = format_log("Secret Key Load Failed", &[("Context", "Symmetric Decrypt"), ("Key Path", key_path_str), ("Error", &e)]);
            write_log_entry(app_handle, &msg).ok();
            return Err(msg);
        }
    };

    let encrypted_data = fs::read(input_path)
        .map_err(|e| {
            let msg = format_log("File Read Failed", &[("Path", input_path), ("Error", &e.to_string())]);
            write_log_entry(app_handle, &msg).ok();
            msg
        })?;

    const NONCE_LEN: usize = 12;
    if encrypted_data.len() < NONCE_LEN {
        let msg = format_log("Decrypt Failed", &[("Reason", "Corrupt/Short File"), ("File Size", &encrypted_data.len().to_string())]);
        write_log_entry(app_handle, &msg).ok();
        return Err(msg);
    }

    let (nonce_slice, ciphertext_with_tag) = encrypted_data.split_at(NONCE_LEN);
    let nonce = Nonce::from_slice(nonce_slice);

    let cipher = Aes256Gcm::new(&key);

    let decrypted_data = cipher.decrypt(nonce, ciphertext_with_tag.as_ref())
        .map_err(|_| {
            let msg = format_log("Decrypt Failed", &[("Reason", "Authentication Tag or Key Mismatch")]);
            write_log_entry(app_handle, &msg).ok();
            msg
        })?;

    fs::write(output_path, decrypted_data)
        .map_err(|e| {
            let msg = format_log("File Write Failed", &[("Path", output_path), ("Error", &e.to_string())]);
            write_log_entry(app_handle, &msg).ok();
            msg
        })?;

    let success_msg = format!("✅ Symmetric decryption completed successfully.");
    write_log_entry(app_handle, &format_log("Symmetric Decrypt Success", &[("Output Path", output_path)])).ok();

    Ok(success_msg)
}

pub fn asymmetric_encrypt(app_handle: &AppHandle, input_path: &str, output_path: &str) -> Result<String, String> {
    write_log_entry(app_handle, &format_log("Asymmetric Encrypt Start", &[("File", input_path)])).ok();

    let app_data_dir = app_handle.path().app_data_dir()
        .map_err(|e| {
            let msg = format_log("Keys Dir Error", &[("Context", "Asymmetric Encrypt"), ("Error", &e.to_string())]);
            write_log_entry(app_handle, &msg).ok();
            msg
        })?;

    let public_key_path = app_data_dir.join("keys").join("public_key.txt");
    let public_key_path_str = public_key_path.to_str().ok_or("Invalid key path string".to_string())?;

    let plaintext = fs::read(input_path)
        .map_err(|e| {
            let msg = format_log("File Read Failed", &[("Path", input_path), ("Error", &e.to_string())]);
            write_log_entry(app_handle, &msg).ok();
            msg
        })?;

    let public_key = load_public_key(public_key_path_str)
        .map_err(|e| {
            let msg = format_log("Public Key Load Failed", &[("Key Path", public_key_path_str), ("Error", &e)]);
            write_log_entry(app_handle, &msg).ok();
            msg
        })?;

    let mut rng = OsRng;
    let padding = Oaep::new::<Sha256>();

    let ciphertext = public_key.encrypt(&mut rng, padding, &plaintext)
        .map_err(|e| {
            let msg = format_log("Asymmetric Encrypt Failed", &[("Error", &e.to_string())]);
            write_log_entry(app_handle, &msg).ok();
            msg
        })?;

    fs::write(output_path, ciphertext)
        .map_err(|e| {
            let msg = format_log("File Write Failed", &[("Path", output_path), ("Error", &e.to_string())]);
            write_log_entry(app_handle, &msg).ok();
            msg
        })?;

    let success_msg = format!("✅ Asymmetric encryption completed successfully.");
    write_log_entry(app_handle, &format_log("Asymmetric Encrypt Success", &[("Output Path", output_path)])).ok();

    Ok(success_msg)
}

pub fn asymmetric_decrypt(app_handle: &AppHandle, input_path: &str, output_path: &str) -> Result<String, String> {
    write_log_entry(app_handle, &format_log("Asymmetric Decrypt Start", &[("File", input_path)])).ok();

    let app_data_dir = app_handle.path().app_data_dir()
        .map_err(|e| {
            let msg = format_log("Keys Dir Error", &[("Context", "Asymmetric Decrypt"), ("Error", &e.to_string())]);
            write_log_entry(app_handle, &msg).ok();
            msg
        })?;

    let private_key_path = app_data_dir.join("keys").join("private_key.txt");
    let private_key_path_str = private_key_path.to_str().ok_or("Invalid key path string".to_string())?;

    let ciphertext = fs::read(input_path)
        .map_err(|e| {
            let msg = format_log("File Read Failed", &[("Path", input_path), ("Error", &e.to_string())]);
            write_log_entry(app_handle, &msg).ok();
            msg
        })?;

    let private_key = load_private_key(private_key_path_str)
        .map_err(|e| {
            let msg = format_log("Private Key Load Failed", &[("Key Path", private_key_path_str), ("Error", &e)]);
            write_log_entry(app_handle, &msg).ok();
            msg
        })?;
        
    let padding = Oaep::new::<Sha256>();

    let decrypted_data = private_key.decrypt(padding, ciphertext.as_ref())
        .map_err(|e| {
            let msg = format_log("Asymmetric Decrypt Failed", &[("Error", &e.to_string())]);
            write_log_entry(app_handle, &msg).ok();
            msg
        })?;

    fs::write(output_path, decrypted_data)
        .map_err(|e| {
            let msg = format_log("File Write Failed", &[("Path", output_path), ("Error", &e.to_string())]);
            write_log_entry(app_handle, &msg).ok();
            msg
        })?;

    let success_msg = format!("✅ Asymmetric decryption completed successfully.");
    write_log_entry(app_handle, &format_log("Asymmetric Decrypt Success", &[("Output Path", output_path)])).ok();

    Ok(success_msg)
}
