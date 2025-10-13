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

pub fn calculate_digest_and_save(app_handle: &tauri::AppHandle, input_path: &str) -> Result<String, String> {
    let digest = calculate_digest(Path::new(input_path))?;
    
    let digest_hex = hex::encode(digest);
    
    let app_data_dir = app_handle.path().app_data_dir()
        .map_err(|e| format!("Error reading app data directory: {}", e))?;

    let digest_path = app_data_dir.join("digest.txt");
    
    fs::write(&digest_path, digest_hex)
        .map_err(|e| format!("Error saving digest: {}", e))?;

    Ok(format!("Digest successfully created and saved to: {}", digest_path.display()))
}


pub fn digitally_sign(app_handle: &tauri::AppHandle, input_file_path: &str) -> Result<String, String> {
    
    let app_data_dir = app_handle.path().app_data_dir()
        .map_err(|e| format!("Error opening app data directory: {}", e))?;

    let priv_key_path = app_data_dir.join("keys").join("private_key.txt");
    let signature_path = app_data_dir.join("digital_signature.txt");
    
    let private_key = load_private_key(priv_key_path.to_str().unwrap_or_default())?;

    let digest = calculate_digest(Path::new(input_file_path))?;

    let mut rng = OsRng;
    let signing_key = SigningKey::<Sha256>::new(private_key);

    let signature_bytes = signing_key
        .try_sign_with_rng(&mut rng, &digest)
        .map_err(|e| format!("Error digitally signing the digest: {}", e))?
        .to_bytes()
        .to_vec();

    let signature_base64 = general_purpose::STANDARD.encode(&signature_bytes);
    
    fs::write(&signature_path, &signature_base64)
        .map_err(|e| format!("Error saving digital signature: {}", e))?;

    Ok(format!("Signature successfully created and saved to: {}", signature_path.display()))
}


pub fn verify_signature(app_handle: &tauri::AppHandle, file_path: &str, signature_path: &str) -> Result<bool, String> {
    
    let app_data_dir = app_handle.path().app_data_dir()
        .map_err(|e| format!("Error opening app data directory: {}", e))?;

    let pub_key_path = app_data_dir.join("keys").join("public_key.txt");
    
    let public_key = load_public_key(pub_key_path.to_str().unwrap_or_default())?;

    let signature_base64 = fs::read_to_string(signature_path)
        .map_err(|e| format!("Error reading signature file: {}", e))?;
    
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