use rsa::pkcs8::{EncodePrivateKey, EncodePublicKey, LineEnding, DecodePrivateKey, DecodePublicKey}; 
use rsa::{RsaPrivateKey, RsaPublicKey};
use rand::rngs::OsRng;
use std::fs;
use aes_gcm::{KeyInit, Aes256Gcm, Key};
use base64::{Engine as _, engine::general_purpose};
use tauri::{AppHandle, Manager, path::BaseDirectory};

use crate::logger::logger::write_log_entry;

const RSA_KEY_SIZE: usize = 2048;

fn format_log(event: &str, details: &[(&str, &str)]) -> String {
    let details_str = details
        .iter()
        .map(|(k, v)| format!("{}: {}", k, v))
        .collect::<Vec<String>>()
        .join("; ");
    format!("EVENT: {}; {}", event, details_str)
}

pub fn generate_and_save(app_handle: AppHandle) -> Result<String, String> {
    write_log_entry(&app_handle, &format_log("Key Generation Start", &[("Algorithm", "RSA & AES-256")])).ok();

    let mut rng = OsRng;

    let private_key = RsaPrivateKey::new(&mut rng, RSA_KEY_SIZE)
        .map_err(|e| {
            let msg = format_log("Key Gen Error", &[("Key Type", "Private RSA"), ("Error", &e.to_string())]);
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
            let msg = format_log("Dir Resolve Error", &[("Dir", "keys"), ("Error", &e.to_string())]);
            write_log_entry(&app_handle, &msg).ok();
            msg
        })?;

    fs::create_dir_all(&base_path)
        .map_err(|e| {
            let msg = format_log("Dir Creation Error", &[("Dir", &base_path.to_string_lossy()), ("Error", &e.to_string())]);
            write_log_entry(&app_handle, &msg).ok();
            msg
        })?;

    private_key
        .write_pkcs8_pem_file(base_path.join("private_key.txt"), LineEnding::LF)
        .map_err(|e| {
            let msg = format_log("Key Save Error", &[("Key Type", "Private RSA"), ("Error", &e.to_string())]);
            write_log_entry(&app_handle, &msg).ok();
            msg
        })?;

    public_key
        .write_public_key_pem_file(base_path.join("public_key.txt"), LineEnding::LF)
        .map_err(|e| {
            let msg = format_log("Key Save Error", &[("Key Type", "Public RSA"), ("Error", &e.to_string())]);
            write_log_entry(&app_handle, &msg).ok();
            msg
        })?;

    fs::write(base_path.join("secret_key.txt"), aes_key_base64)
        .map_err(|e| {
            let msg = format_log("Key Save Error", &[("Key Type", "Secret AES"), ("Error", &e.to_string())]);
            write_log_entry(&app_handle, &msg).ok();
            msg
        })?;

    let success_msg = format!("✅ Keys successfully generated and saved.");
    write_log_entry(&app_handle, &format_log("Key Generation Success", &[("Location", &base_path.to_string_lossy()), ("Size", &RSA_KEY_SIZE.to_string())])).ok();

    Ok(success_msg)
}

pub fn load_private_key(path: &str) -> Result<RsaPrivateKey, String> {
    RsaPrivateKey::read_pkcs8_pem_file(path)
        .map_err(|e| {
            format!("Error loading private key: {}", e) 
        })
}

pub fn load_public_key(path: &str) -> Result<RsaPublicKey, String> {
    RsaPublicKey::read_public_key_pem_file(path)
        .map_err(|e| {
            format!("Error loading public key: {}", e)
        })
}

pub fn load_secret_key(path: &str) -> Result<Key<Aes256Gcm>, String> {
    let key_base64 = fs::read_to_string(path)
        .map_err(|e| {
            format!("Error reading secret key from disk: {}", e)
        })?;
    
    let key_bytes = general_purpose::STANDARD.decode(key_base64.trim())
        .map_err(|_| {
            "Error decoding secret key from Base64 format".to_string()
        })?;

    if key_bytes.len() != 32 {
        return Err(format!("Secret key must be 32 bytes long (256 bits). Found {} bytes after Base64 decoding.", key_bytes.len()));
    }

    Ok(*Key::<Aes256Gcm>::from_slice(&key_bytes))
}

fn keys_base_path(app_handle: &AppHandle) -> Result<std::path::PathBuf, String> {
    app_handle
        .path()
        .resolve("keys", BaseDirectory::AppData)
        .map_err(|e| {
            let msg = format_log("Dir Resolve Error", &[("Dir", "keys"), ("Error", &e.to_string())]);
            write_log_entry(app_handle, &msg).ok();
            msg
        })
}

pub fn load_private_key_for_display(app_handle: &AppHandle) -> Result<String, String> {
    let path = keys_base_path(app_handle)?.join("private_key.txt");
    let pem = fs::read_to_string(&path)
        .map_err(|e| {
            let msg = format_log("Key Display Error", &[("Key Type", "Private RSA"), ("Action", "Read"), ("Error", &e.to_string())]);
            write_log_entry(app_handle, &msg).ok();
            msg
        })?;
    write_log_entry(app_handle, &format_log("Key Display Success", &[("Key Type", "Private RSA")])).ok();
    Ok(pem)
}

pub fn load_public_key_for_display(app_handle: &AppHandle) -> Result<String, String> {
    let path = keys_base_path(app_handle)?.join("public_key.txt");
    let pem = fs::read_to_string(&path)
        .map_err(|e| {
            let msg = format_log("Key Display Error", &[("Key Type", "Public RSA"), ("Action", "Read"), ("Error", &e.to_string())]);
            write_log_entry(app_handle, &msg).ok();
            msg
        })?;
    write_log_entry(app_handle, &format_log("Key Display Success", &[("Key Type", "Public RSA")])).ok();
    Ok(pem)
}

pub fn load_secret_key_for_display(app_handle: &AppHandle) -> Result<String, String> {
    let path = keys_base_path(app_handle)?.join("secret_key.txt");
    let key_base64 = fs::read_to_string(&path)
        .map_err(|e| {
            let msg = format_log("Key Display Error", &[("Key Type", "Secret AES"), ("Action", "Read"), ("Error", &e.to_string())]);
            write_log_entry(app_handle, &msg).ok();
            msg
        })?;
    write_log_entry(app_handle, &format_log("Key Display Success", &[("Key Type", "Secret AES")])).ok();
    Ok(key_base64.trim().to_string())
}