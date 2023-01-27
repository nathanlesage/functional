#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::env;

// Import our commands
mod commands;
use crate::commands::{list_plugins, get_plugin_js};

// Import our system tray
mod system_tray;
use crate::system_tray::{create_system_tray, handle_system_tray_events};

fn main() {
  // Setup any variables and objects we need for booting the app
  let system_tray = create_system_tray();

  // Now boot the application.
  tauri::Builder::default()
    // Configure system tray
    .system_tray(system_tray)
    .on_system_tray_event(handle_system_tray_events)

    // Build our event handlers
    .invoke_handler(tauri::generate_handler![list_plugins, get_plugin_js])

    // Start the app
    .build(tauri::generate_context!())
    .expect("Could not build application")

    // Handle app events
    .run(|_app_handle, event| match event {
      // Prevent exiting normally (i.e. on Window close). Users will have
      // to actually click the Quit item since this is a tray app.
      // NOTE: To programmatically quit, one can simply quit the process (see
      // tray menu implementation).
      tauri::RunEvent::ExitRequested { api, .. } => {
        api.prevent_exit();
      }
      _ => {}
    });
}
