use rsa::pkcs8::{EncodePrivateKey, EncodePublicKey, LineEnding, DecodePrivateKey, DecodePublicKey}; 
use rsa::{RsaPrivateKey, RsaPublicKey};
use rand::rngs::OsRng;
use std::fs;
use std::path::PathBuf;
use aes_gcm::{KeyInit, Aes256Gcm, Key};
use base64::{Engine as _, engine::general_purpose};

const RSA_KEY_SIZE: usize = 2048;

pub fn generate_and_save() -> Result<String, String> {
    let mut rng = OsRng;
    
    let private_key = RsaPrivateKey::new(&mut rng, RSA_KEY_SIZE)
        .map_err(|e| format!("Error generating RSA private key: {}", e))?;
    let public_key = RsaPublicKey::from(&private_key);

    let aes_key = Aes256Gcm::generate_key(&mut rng);
    let aes_key_base64 = general_purpose::STANDARD.encode(aes_key);


    let base_path = PathBuf::from("./keys");
    fs::create_dir_all(&base_path).map_err(|e| format!("Error creating keys directory: {}", e))?;

    let priv_path = base_path.join("private_key.txt");
    let pub_path = base_path.join("public_key.txt");
    let secret_path = base_path.join("secret_key.txt");

    private_key.write_pkcs8_pem_file(priv_path, LineEnding::LF)
        .map_err(|e| format!("Error saving private key: {}", e))?;

    public_key.write_public_key_pem_file(pub_path, LineEnding::LF)
        .map_err(|e| format!("Error saving public key: {}", e))?;

    fs::write(secret_path, aes_key_base64)
        .map_err(|e| format!("Error saving secret key: {}", e))?;


    Ok(format!("Keys successfully generated and saved to the './keys/' directory."))
}

pub fn load_private_key(path: &str) -> Result<RsaPrivateKey, String> {
    RsaPrivateKey::read_pkcs8_pem_file(path)
        .map_err(|e| format!("Error loading private key: {}", e))
}

pub fn load_public_key(path: &str) -> Result<RsaPublicKey, String> {
    RsaPublicKey::read_public_key_pem_file(path)
        .map_err(|e| format!("Error loading public key: {}", e))
}

pub fn load_secret_key(path: &str) -> Result<Key<Aes256Gcm>, String> {
    let key_base64 = fs::read_to_string(path)
        .map_err(|e| format!("Error reading secret key from disk: {}", e))?;
    
    let key_bytes = general_purpose::STANDARD.decode(key_base64.trim())
        .map_err(|_| "Error decoding secret key from Base64 format".to_string())?;

    if key_bytes.len() != 32 {
        return Err(format!("Secret key must be 32 bytes long (256 bits). Found {} bytes after Base64 decoding.", key_bytes.len()));
    }
    
    Ok(*Key::<Aes256Gcm>::from_slice(&key_bytes))
}