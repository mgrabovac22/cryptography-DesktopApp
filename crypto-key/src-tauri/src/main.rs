#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod crypto;
mod logger;

use commands::{
    keys::{generate_keys, get_private_key, get_public_key, get_secret_key},
    signature::{calculate_digest_and_save, digitally_sign, verify_signature, list_signatures_cmd},
    crypting::{symmetric_encrypt, symmetric_decrypt, asymmetric_encrypt, asymmetric_decrypt},
    logger::read_logs_command,
};
use tauri::{
    Manager,
    menu::{MenuBuilder, MenuItemBuilder, SubmenuBuilder},
    Emitter
};

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
            let handle = app.app_handle();
            match event.id().as_ref() {
                "open_app" => { let _ = handle.emit("menu-open-app", ()); }
                "open_logger" => { let _ = handle.emit("menu-open-logger", ()); }
                "snow_increase" => { let _ = handle.emit("snow-increase", ()); }
                "snow_decrease" => { let _ = handle.emit("snow-decrease", ()); }
                "snow_toggle" => { let _ = handle.emit("snow-toggle", ()); }
                "show_about" => { let _ = handle.emit("menu-open-about", ()); }
                "show_dev" => { let _ = handle.emit("menu-open-developer", ()); }
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![
            generate_keys,
            get_private_key,
            get_public_key,
            get_secret_key,
            calculate_digest_and_save,
            digitally_sign,
            verify_signature,
            list_signatures_cmd,
            symmetric_encrypt,
            symmetric_decrypt,
            asymmetric_encrypt,
            asymmetric_decrypt,
            read_logs_command
        ])
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("Error while running the Tauri application.");
}
