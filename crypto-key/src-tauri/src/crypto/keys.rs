use rsa::pkcs8::{EncodePrivateKey, EncodePublicKey, LineEnding, DecodePrivateKey, DecodePublicKey}; 
use rsa::{RsaPrivateKey, RsaPublicKey};
use rand::rngs::OsRng;
use std::fs;
use aes_gcm::{KeyInit, Aes256Gcm, Key};
use base64::{Engine as _, engine::general_purpose};
use tauri::{AppHandle, Manager, path::BaseDirectory};

use crate::logger::logger::write_log_entry;

const RSA_KEY_SIZE: usize = 2048;

pub fn generate_and_save(app_handle: AppHandle) -> Result<String, String> {
    write_log_entry(&app_handle, "Starting key generation").ok();

    let mut rng = OsRng;

    let private_key = RsaPrivateKey::new(&mut rng, RSA_KEY_SIZE)
        .map_err(|e| {
            let msg = format!("Error generating RSA private key: {}", e);
            write_log_entry(&app_handle, &msg).ok();
            msg
        })?;
    let public_key = RsaPublicKey::from(&private_key);

    let aes_key = Aes256Gcm::generate_key(&mut rng);
    let aes_key_base64 = general_purpose::STANDARD.encode(aes_key);

    let base_path = app_handle
        .path()
        .resolve("keys", BaseDirectory::AppData)
        .map_err(|e| {
            let msg = format!("Error resolving app data dir: {}", e);
            write_log_entry(&app_handle, &msg).ok();
            msg
        })?;

    fs::create_dir_all(&base_path)
        .map_err(|e| {
            let msg = format!("Error creating keys directory: {}", e);
            write_log_entry(&app_handle, &msg).ok();
            msg
        })?;

    private_key
        .write_pkcs8_pem_file(base_path.join("private_key.txt"), LineEnding::LF)
        .map_err(|e| {
            let msg = format!("Error saving private key: {}", e);
            write_log_entry(&app_handle, &msg).ok();
            msg
        })?;

    public_key
        .write_public_key_pem_file(base_path.join("public_key.txt"), LineEnding::LF)
        .map_err(|e| {
            let msg = format!("Error saving public key: {}", e);
            write_log_entry(&app_handle, &msg).ok();
            msg
        })?;

    fs::write(base_path.join("secret_key.txt"), aes_key_base64)
        .map_err(|e| {
            let msg = format!("Error saving secret key: {}", e);
            write_log_entry(&app_handle, &msg).ok();
            msg
        })?;

    let success_msg = format!("✅ Keys successfully generated and saved.");
    write_log_entry(&app_handle, &success_msg).ok();

    Ok(success_msg)
}

pub fn load_private_key(path: &str) -> Result<RsaPrivateKey, String> {
    RsaPrivateKey::read_pkcs8_pem_file(path)
        .map_err(|e| {
            let msg = format!("Error loading private key: {}", e);
            msg
        })
}

pub fn load_public_key(path: &str) -> Result<RsaPublicKey, String> {
    RsaPublicKey::read_public_key_pem_file(path)
        .map_err(|e| {
            let msg = format!("Error loading public key: {}", e);
            msg
        })
}

pub fn load_secret_key(path: &str) -> Result<Key<Aes256Gcm>, String> {
    let key_base64 = fs::read_to_string(path)
        .map_err(|e| {
            let msg = format!("Error reading secret key from disk: {}", e);
            msg
        })?;
    
    let key_bytes = general_purpose::STANDARD.decode(key_base64.trim())
        .map_err(|_| {
            let msg = "Error decoding secret key from Base64 format".to_string();
            msg
        })?;

    if key_bytes.len() != 32 {
        let msg = format!("Secret key must be 32 bytes long (256 bits). Found {} bytes after Base64 decoding.", key_bytes.len());
        return Err(msg);
    }

    Ok(*Key::<Aes256Gcm>::from_slice(&key_bytes))
}

fn keys_base_path(app_handle: &AppHandle) -> Result<std::path::PathBuf, String> {
    app_handle
        .path()
        .resolve("keys", BaseDirectory::AppData)
        .map_err(|e| {
            let msg = format!("Error resolving keys directory: {}", e);
            write_log_entry(app_handle, &msg).ok();
            msg
        })
}

pub fn load_private_key_for_display(app_handle: &AppHandle) -> Result<String, String> {
    let path = keys_base_path(app_handle)?.join("private_key.txt");
    let pem = fs::read_to_string(&path)
        .map_err(|e| {
            let msg = format!("Error reading private key for display: {}", e);
            write_log_entry(app_handle, &msg).ok();
            msg
        })?;
    Ok(pem)
}

pub fn load_public_key_for_display(app_handle: &AppHandle) -> Result<String, String> {
    let path = keys_base_path(app_handle)?.join("public_key.txt");
    let pem = fs::read_to_string(&path)
        .map_err(|e| {
            let msg = format!("Error reading public key for display: {}", e);
            write_log_entry(app_handle, &msg).ok();
            msg
        })?;
    Ok(pem)
}

pub fn load_secret_key_for_display(app_handle: &AppHandle) -> Result<String, String> {
    let path = keys_base_path(app_handle)?.join("secret_key.txt");
    let key_base64 = fs::read_to_string(&path)
        .map_err(|e| {
            let msg = format!("Error reading secret key for display: {}", e);
            write_log_entry(app_handle, &msg).ok();
            msg
        })?;
    Ok(key_base64.trim().to_string())
}
