#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod crypto;
mod logger;

use crypto::{keys, signature, crypting};
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder, SubmenuBuilder},
    Emitter,
};

#[tauri::command]
fn generate_keys(app_handle: tauri::AppHandle) -> Result<String, String> {
    keys::generate_and_save(app_handle)
}

#[tauri::command]
fn calculate_digest_and_save(app_handle: tauri::AppHandle, input_path: String) -> Result<String, String> {
    signature::calculate_digest_and_save(&app_handle, &input_path)
}

#[tauri::command]
fn digitally_sign(app_handle: tauri::AppHandle, file_path: String) -> Result<String, String> {
    signature::digitally_sign(&app_handle, &file_path)
}

#[tauri::command]
fn verify_signature(app_handle: tauri::AppHandle, file_path: String, signature_path: String) -> Result<bool, String> {
    signature::verify_signature(&app_handle, &file_path, &signature_path)
}

#[tauri::command]
fn symmetric_encrypt(app_handle: tauri::AppHandle, input_path: String, output_path: String) -> Result<String, String> {
    crypting::symmetric_encrypt(&app_handle, input_path, output_path)
}

#[tauri::command]
fn symmetric_decrypt(app_handle: tauri::AppHandle, input_path: String, output_path: String) -> Result<String, String> {
    crypting::symmetric_decrypt(&app_handle, &input_path, &output_path)
}

#[tauri::command]
fn asymmetric_encrypt(app_handle: tauri::AppHandle, input_path: String, output_path: String) -> Result<String, String> {
    crypting::asymmetric_encrypt(&app_handle, &input_path, &output_path)
}

#[tauri::command]
fn asymmetric_decrypt(app_handle: tauri::AppHandle, input_path: String, output_path: String) -> Result<String, String> {
    crypting::asymmetric_decrypt(&app_handle, &input_path, &output_path)
}

#[tauri::command]
fn list_signatures_cmd(app_handle: tauri::AppHandle) -> Result<Vec<String>, String> {
    signature::list_signatures(&app_handle)
}

#[tauri::command]
fn read_logs_command(app_handle: tauri::AppHandle) -> Result<String, String> {
    logger::logger::read_logs(&app_handle)
}

#[tauri::command]
fn get_private_key(app_handle: tauri::AppHandle) -> Result<String, String> {
    keys::load_private_key_for_display(&app_handle)
}

#[tauri::command]
fn get_public_key(app_handle: tauri::AppHandle) -> Result<String, String> {
    keys::load_public_key_for_display(&app_handle)
}

#[tauri::command]
fn get_secret_key(app_handle: tauri::AppHandle) -> Result<String, String> {
    keys::load_secret_key_for_display(&app_handle)
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            logger::logger::init_logger(&app.handle());
            Ok(())
        })

        .menu(|app| {
            let view_menu = SubmenuBuilder::new(app, "View")
                .item(&MenuItemBuilder::new("App").id("open_app").build(app)?)
                .item(&MenuItemBuilder::new("Log data").id("open_logger").build(app)?)
                .build()?;

            let snow_menu = SubmenuBuilder::new(app, "Snow Control")
                .item(&MenuItemBuilder::new("Increase Intensity").id("snow_increase").build(app)?)
                .item(&MenuItemBuilder::new("Decrease Intensity").id("snow_decrease").build(app)?)
                .item(&MenuItemBuilder::new("Toggle Snow").id("snow_toggle").build(app)?)
                .build()?;

            let info_menu = SubmenuBuilder::new(app, "Info")
                .item(&MenuItemBuilder::new("About App").id("show_about").build(app)?)
                .item(&MenuItemBuilder::new("Developer").id("show_dev").build(app)?)
                .build()?;

            MenuBuilder::new(app)
                .items(&[&view_menu, &snow_menu, &info_menu])
                .build()
        })

        .on_menu_event(|app, event| {
            match event.id().as_ref() {
                "open_app" => {
                    let _ = app.emit("menu-open-app", ());
                }
                "open_logger" => {
                    let _ = app.emit("menu-open-logger", ());
                }
                "snow_increase" => {
                    let _ = app.emit("snow-increase", ());
                }
                "snow_decrease" => {
                    let _ = app.emit("snow-decrease", ());
                }
                "snow_toggle" => {
                    let _ = app.emit("snow-toggle", ());
                }
                "show_about" => {
                    let _ = app.emit("menu-open-about", ());
                }
                "show_dev" => {
                    let _ = app.emit("menu-open-developer", ());
                }
                _ => {}
            }
        })

        .invoke_handler(tauri::generate_handler![
            generate_keys,
            calculate_digest_and_save,
            digitally_sign,
            verify_signature,
            symmetric_encrypt,
            symmetric_decrypt,
            asymmetric_encrypt,
            asymmetric_decrypt,
            list_signatures_cmd,
            read_logs_command,
            get_private_key,
            get_public_key,
            get_secret_key
        ])
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("Error while running the Tauri application.");
}