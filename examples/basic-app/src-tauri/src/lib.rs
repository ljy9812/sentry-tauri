#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use sentry;
use tauri_plugin_sentry;

#[tauri::command]
fn rust_breadcrumb() {
    sentry::add_breadcrumb(sentry::Breadcrumb {
        message: Some("This is a breadcrumb from Rust".to_owned()),
        ..Default::default()
    })
}

#[tauri::command]
fn rust_panic() {
    panic!("This is a panic from Rust");
}

#[cfg(all(not(target_os = "ios"), not(target_env = "ohos")))]
#[tauri::command]
fn native_crash() {
    unsafe { sadness_generator::raise_segfault() }
}

#[cfg_attr(any(mobile, target_env = "ohos"), tauri::mobile_entry_point)]
pub fn run() {
    let client = sentry::init((
        option_env!("SENTRY_DSN").unwrap_or(""),
        sentry::ClientOptions {
            release: sentry::release_name!(),
            debug: true,
            ..Default::default()
        },
    ));

    #[cfg(all(not(target_os = "ios"), not(target_env = "ohos")))]
    let _guard = tauri_plugin_sentry::minidump::init(&client);

    tauri::Builder::default()
        .plugin(tauri_plugin_sentry::init(&client))
        .invoke_handler(tauri::generate_handler![
            rust_breadcrumb,
            rust_panic,
            #[cfg(all(not(target_os = "ios"), not(target_env = "ohos")))]
            native_crash
        ])
        .run(tauri::generate_context!())
        .expect("error while starting tauri app");
}
