use std::fs;
use std::path::{Path, PathBuf};
use sha2::{Sha256, Digest};
use hex;
use rsa::pkcs1v15::{SigningKey, VerifyingKey, Signature as Pkcs1v15Signature};
use rsa::signature::{Verifier, RandomizedSigner, SignatureEncoding};
use base64::{Engine as _, engine::general_purpose};
use rand::rngs::OsRng;
use tauri::Manager;
use tauri::AppHandle;

use super::keys::{load_private_key, load_public_key}; 


fn calculate_digest(path: &Path) -> Result<Vec<u8>, String> {
    let mut file = fs::File::open(path)
        .map_err(|e| format!("Error opening file {}: {}", path.display(), e))?;

    let mut hasher = Sha256::new();
    std::io::copy(&mut file, &mut hasher)
        .map_err(|e| format!("Error reading file for hashing: {}", e))?;

    Ok(hasher.finalize().to_vec())
}

pub fn calculate_digest_and_save(app_handle: &AppHandle, input_path: &str) -> Result<String, String> {
    let digest = calculate_digest(Path::new(input_path))?;
    let digest_hex = hex::encode(digest);

    let base_path = app_handle
        .path()
        .resolve("digest", tauri::path::BaseDirectory::AppData)
        .map_err(|e| format!("Error resolving digest directory: {}", e))?;

    fs::create_dir_all(&base_path)
        .map_err(|e| format!("Error creating digest directory: {}", e))?;

    let digest_path = base_path.join("digest.txt");
    fs::write(&digest_path, digest_hex)
        .map_err(|e| format!("Error saving digest file: {}", e))?;

    Ok(format!(
        "Digest successfully created and saved to: {}",
        digest_path.display()
    ))
}

pub fn digitally_sign(app_handle: &AppHandle, input_file_path: &str) -> Result<String, String> {
    let base_path = app_handle
        .path()
        .resolve("keys", tauri::path::BaseDirectory::AppData)
        .map_err(|e| format!("Error resolving keys directory: {}", e))?;

    let priv_key_path = base_path.join("private_key.txt");
    let private_key = load_private_key(priv_key_path.to_str().unwrap_or_default())?;

    let digest = calculate_digest(Path::new(input_file_path))?;

    let mut rng = OsRng;
    let signing_key = rsa::pkcs1v15::SigningKey::<Sha256>::new(private_key);
    let signature = signing_key
        .try_sign_with_rng(&mut rng, &digest)
        .map_err(|e| format!("Error digitally signing digest: {}", e))?;

    let signature_base64 = general_purpose::STANDARD.encode(signature);

    let sig_dir = app_handle
        .path()
        .resolve("signature", tauri::path::BaseDirectory::AppData)
        .map_err(|e| format!("Error resolving signature directory: {}", e))?;

    fs::create_dir_all(&sig_dir)
        .map_err(|e| format!("Error creating signature directory: {}", e))?;

    let signature_path = sig_dir.join("digital_signature.txt");
    fs::write(&signature_path, &signature_base64)
        .map_err(|e| format!("Error saving digital signature: {}", e))?;

    Ok(format!(
        "Signature successfully created and saved to: {}",
        signature_path.display()
    ))
}

pub fn verify_signature(app_handle: &tauri::AppHandle, file_path: &str, signature_path: &str) -> Result<bool, String> {
    
    let app_data_dir = app_handle.path().app_data_dir()
        .map_err(|e| format!("Error opening app data directory: {}", e))?;

    let pub_key_path = app_data_dir.join("keys").join("public_key.txt");
    
    let public_key = load_public_key(pub_key_path.to_str().unwrap_or_default())?;

    let sig_full_path = app_data_dir.join(signature_path);
    let signature_base64 = fs::read_to_string(&sig_full_path)
        .map_err(|e| format!("Error reading signature file ({}): {}", sig_full_path.display(), e))?;
    
    let signature_bytes = general_purpose::STANDARD.decode(signature_base64.trim())
        .map_err(|_| "Error decoding signature from Base64 format".to_string())?;

    let signature = Pkcs1v15Signature::try_from(signature_bytes.as_slice())
        .map_err(|_| "Error converting signature bytes to Signature type".to_string())?;

    let digest_from_file = calculate_digest(Path::new(file_path))?;

    let verifying_key = VerifyingKey::<Sha256>::new(public_key);

    match verifying_key.verify(&digest_from_file, &signature) {
        Ok(_) => Ok(true),
        Err(e) => Err(format!("Signature INVALID: The file was modified or the signature is fraudulent. (Error: {})", e)),
    }
}

pub fn list_signatures(app_handle: &AppHandle) -> Result<Vec<String>, String> {
    let sig_dir = app_handle
        .path()
        .resolve("signature", tauri::path::BaseDirectory::AppData)
        .map_err(|e| format!("Error resolving signature directory: {}", e))?;

    if !sig_dir.exists() {
        return Ok(vec![]);
    }

    let mut signatures = Vec::new();
    for entry in fs::read_dir(&sig_dir)
        .map_err(|e| format!("Error reading signature directory: {}", e))?
    {
        let entry = entry.map_err(|e| format!("Error reading file entry: {}", e))?;
        let path = entry.path();

        if path.is_file() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                signatures.push(name.to_string());
            }
        }
    }

    signatures.sort();

    Ok(signatures)
}