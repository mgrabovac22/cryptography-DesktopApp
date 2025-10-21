use std::fs;
use std::path::{Path};
use sha2::{Sha256, Digest};
use hex;
use rsa::pkcs1v15::{VerifyingKey, Signature as Pkcs1v15Signature};
use rsa::signature::{Verifier, RandomizedSigner};
use base64::{Engine as _, engine::general_purpose};
use rand::rngs::OsRng;
use tauri::{Manager, AppHandle};

use super::keys::{load_private_key, load_public_key}; 
use crate::logger::logger::write_log_entry;

fn format_log(event: &str, details: &[(&str, &str)]) -> String {
    let details_str = details
        .iter()
        .map(|(k, v)| format!("{}: {}", k, v))
        .collect::<Vec<String>>()
        .join("; ");
    format!("EVENT: {}; {}", event, details_str)
}

fn calculate_digest(path: &Path) -> Result<Vec<u8>, String> {
    let path_str = path.to_string_lossy();
    
    let mut file = fs::File::open(path)
        .map_err(|e| format_log("File Open Error", &[("File", &path_str), ("Error", &e.to_string())]))?;

    let mut hasher = Sha256::new();
    std::io::copy(&mut file, &mut hasher)
        .map_err(|e| format_log("File Read Error", &[("File", &path_str), ("Error", &e.to_string())]))?;

    Ok(hasher.finalize().to_vec())
}

pub fn calculate_digest_and_save(app_handle: &AppHandle, input_path: &str) -> Result<String, String> {
    write_log_entry(app_handle, &format_log("Digest Calculation Start", &[("File", input_path)])).ok();

    let digest_result = calculate_digest(Path::new(input_path));
    let digest = match digest_result {
        Ok(d) => d,
        Err(e) => {
            write_log_entry(app_handle, &e).ok();
            return Err(e);
        }
    };
    
    let digest_hex = hex::encode(digest);

    let base_path = app_handle
        .path()
        .resolve("digest", tauri::path::BaseDirectory::AppData)
        .map_err(|e| {
            let msg = format_log("Dir Resolve Error", &[("Dir", "digest"), ("Error", &e.to_string())]);
            write_log_entry(app_handle, &msg).ok();
            msg
        })?;

    fs::create_dir_all(&base_path)
        .map_err(|e| {
            let msg = format_log("Dir Creation Error", &[("Dir", &base_path.to_string_lossy()), ("Error", &e.to_string())]);
            write_log_entry(app_handle, &msg).ok();
            msg
        })?;

    let digest_path = base_path.join("digest.txt");
    fs::write(&digest_path, &digest_hex)
        .map_err(|e| {
            let msg = format_log("Digest Save Error", &[("Path", &digest_path.to_string_lossy()), ("Error", &e.to_string())]);
            write_log_entry(app_handle, &msg).ok();
            msg
        })?;

    let success_msg = format!("✅ Digest successfully created and saved.");
    write_log_entry(app_handle, &format_log("Digest Calculation Success", &[("File", input_path), ("Digest Length", &digest_hex.len().to_string())])).ok();

    Ok(success_msg)
}

pub fn digitally_sign(app_handle: &AppHandle, input_file_path: &str) -> Result<String, String> {
    write_log_entry(app_handle, &format_log("Digital Sign Start", &[("File", input_file_path)])).ok();

    let base_path = app_handle
        .path()
        .resolve("keys", tauri::path::BaseDirectory::AppData)
        .map_err(|e| {
            let msg = format_log("Dir Resolve Error", &[("Dir", "keys"), ("Error", &e.to_string())]);
            write_log_entry(app_handle, &msg).ok();
            msg
        })?;

    let priv_key_path = base_path.join("private_key.txt");
    let priv_key_path_str = priv_key_path.to_str().unwrap_or_default();
    
    let private_key = load_private_key(priv_key_path_str)
        .map_err(|e| {
            let msg = format_log("Private Key Load Failed", &[("Path", priv_key_path_str), ("Error", &e)]);
            write_log_entry(app_handle, &msg).ok();
            msg
        })?;

    let digest_result = calculate_digest(Path::new(input_file_path));
    let digest = match digest_result {
        Ok(d) => d,
        Err(e) => {
            write_log_entry(app_handle, &e).ok();
            return Err(e);
        }
    };

    let mut rng = OsRng;
    let signing_key = rsa::pkcs1v15::SigningKey::<Sha256>::new(private_key);
    let signature = signing_key
        .try_sign_with_rng(&mut rng, &digest)
        .map_err(|e| {
            let msg = format_log("Signing Failed", &[("File", input_file_path), ("Error", &e.to_string())]);
            write_log_entry(app_handle, &msg).ok();
            msg
        })?;

    let signature_base64 = general_purpose::STANDARD.encode(signature);

    let sig_dir = app_handle
        .path()
        .resolve("signature", tauri::path::BaseDirectory::AppData)
        .map_err(|e| {
            let msg = format_log("Dir Resolve Error", &[("Dir", "signature"), ("Error", &e.to_string())]);
            write_log_entry(app_handle, &msg).ok();
            msg
        })?;

    fs::create_dir_all(&sig_dir)
        .map_err(|e| {
            let msg = format_log("Dir Creation Error", &[("Dir", &sig_dir.to_string_lossy()), ("Error", &e.to_string())]);
            write_log_entry(app_handle, &msg).ok();
            msg
        })?;

    let signature_path = sig_dir.join("digital_signature.txt");
    fs::write(&signature_path, &signature_base64)
        .map_err(|e| {
            let msg = format_log("Signature Save Error", &[("Path", &signature_path.to_string_lossy()), ("Error", &e.to_string())]);
            write_log_entry(app_handle, &msg).ok();
            msg
        })?;

    let success_msg = format!("✅ Signature successfully created and saved.");
    write_log_entry(app_handle, &format_log("Digital Sign Success", &[("File", input_file_path), ("Sig Path", &signature_path.to_string_lossy())])).ok();

    Ok(success_msg)
}

pub fn verify_signature(app_handle: &tauri::AppHandle, file_path: &str, signature_path: &str) -> Result<bool, String> {
    write_log_entry(app_handle, &format_log("Signature Verify Start", &[("File", file_path), ("Signature File", signature_path)])).ok();

    let app_data_dir = app_handle.path().app_data_dir()
        .map_err(|e| {
            let msg = format_log("Dir Resolve Error", &[("Dir", "AppData"), ("Error", &e.to_string())]);
            write_log_entry(app_handle, &msg).ok();
            msg
        })?;

    let pub_key_path = app_data_dir.join("keys").join("public_key.txt");
    let pub_key_path_str = pub_key_path.to_str().unwrap_or_default();
    
    let public_key = load_public_key(pub_key_path_str)
        .map_err(|e| {
            let msg = format_log("Public Key Load Failed", &[("Path", pub_key_path_str), ("Error", &e)]);
            write_log_entry(app_handle, &msg).ok();
            msg
        })?;

    let sig_full_path = app_data_dir.join(signature_path);
    let sig_full_path_str = sig_full_path.to_string_lossy();
    
    let signature_base64 = fs::read_to_string(&sig_full_path)
        .map_err(|e| {
            let msg = format_log("Signature Read Error", &[("Path", &sig_full_path_str), ("Error", &e.to_string())]);
            write_log_entry(app_handle, &msg).ok();
            msg
        })?;
    
    let signature_bytes = general_purpose::STANDARD.decode(signature_base64.trim())
        .map_err(|_| {
            let msg = format_log("Signature Decode Error", &[("Path", &sig_full_path_str), ("Reason", "Base64 Format Invalid")]);
            write_log_entry(app_handle, &msg).ok();
            msg
        })?;

    let signature = Pkcs1v15Signature::try_from(signature_bytes.as_slice())
        .map_err(|_| {
            let msg = format_log("Signature Convert Error", &[("Path", &sig_full_path_str), ("Reason", "Length/Format Mismatch")]);
            write_log_entry(app_handle, &msg).ok();
            msg
        })?;

    let digest_result = calculate_digest(Path::new(file_path));
    let digest_from_file = match digest_result {
        Ok(d) => d,
        Err(e) => {
            write_log_entry(app_handle, &e).ok();
            return Err(e);
        }
    };

    let verifying_key = VerifyingKey::<Sha256>::new(public_key);

    match verifying_key.verify(&digest_from_file, &signature) {
        Ok(_) => {
            write_log_entry(app_handle, &format_log("Signature Verify SUCCESS", &[("File", file_path)])).ok();
            Ok(true)
        },
        Err(e) => {
            let msg = format_log("Signature Verify FAILED", &[("File", file_path), ("Reason", "Key Mismatch or File Modified"), ("Error", &e.to_string())]);
            write_log_entry(app_handle, &msg).ok();
            Err(msg)
        },
    }
}

pub fn list_signatures(app_handle: &AppHandle) -> Result<Vec<String>, String> {
    write_log_entry(app_handle, &format_log("Signature List Start", &[])).ok();

    let sig_dir = app_handle
        .path()
        .resolve("signature", tauri::path::BaseDirectory::AppData)
        .map_err(|e| {
            let msg = format_log("Dir Resolve Error", &[("Dir", "signature"), ("Error", &e.to_string())]);
            write_log_entry(app_handle, &msg).ok();
            msg
        })?;

    if !sig_dir.exists() {
        write_log_entry(app_handle, &format_log("Signature List Success", &[("Count", "0"), ("Reason", "Directory does not exist")])).ok();
        return Ok(vec![]);
    }

    let read_dir_result = fs::read_dir(&sig_dir)
        .map_err(|e| {
            let msg = format_log("Dir Read Error", &[("Dir", &sig_dir.to_string_lossy()), ("Error", &e.to_string())]);
            write_log_entry(app_handle, &msg).ok();
            msg
        })?;

    let mut signatures = Vec::new();
    for entry in read_dir_result {
        let entry = entry.map_err(|e| {
            let msg = format_log("Dir Entry Read Error", &[("Dir", &sig_dir.to_string_lossy()), ("Error", &e.to_string())]);
            write_log_entry(app_handle, &msg).ok();
            msg
        })?;
        let path = entry.path();

        if path.is_file() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                signatures.push(name.to_string());
            }
        }
    }

    signatures.sort();
    write_log_entry(app_handle, &format_log("Signature List Success", &[("Count", &signatures.len().to_string())])).ok();

    Ok(signatures)
}
