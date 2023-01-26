#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{
  SystemTray,
  SystemTrayEvent,
  SystemTrayMenu,
  SystemTrayMenuItem,
  CustomMenuItem,
  Manager,
  LogicalSize,
  PhysicalPosition
};

use std::fs::{read_dir, metadata, create_dir};
use std::env;
use std::path::Path;

/** Retrieves the data directory (user data in prod, otherwise ./plugins) */
#[cfg(debug_assertions)]
fn retrieve_data_dir() -> String {
  // <root>/src-tauri/target/debug/exe --> <root>/plugins
  let exe = env::current_exe().unwrap();
  let base_path = Path::new(exe.to_str().unwrap());
  let project_root = base_path
    .parent().unwrap() // debug
    .parent().unwrap() // target
    .parent().unwrap() // src-tauri
    .parent().unwrap(); // <root>
  project_root.join("plugins").to_str().unwrap().to_string()
}

#[cfg(not(debug_assertions))]
/// Returns the data directory of the app
fn retrieve_data_dir() -> String {
  let mut data_dir = tauri::api::path::data_dir().unwrap();
  data_dir.push("com.zettlr.functional");
  data_dir.to_str().unwrap().to_string()
}

#[tauri::command]
fn list_plugins() -> Vec<String> {
  let data_dir = retrieve_data_dir();
  let mut plugins: Vec<String> = Vec::new();

  match metadata(&data_dir) {
    Ok(res) => {
      if !res.is_dir() {
        // It's not a directory ...?
        panic!("The path to plugins exists, but it is not a directory?!")
      }
    }
    Err(_) => {
      // Doesn't exist
      create_dir(&data_dir).unwrap();
    }
  }

  for entry_res in read_dir(data_dir).unwrap() {
    match entry_res {
      Ok(entry) => {
        plugins.push(entry.path().to_str().unwrap().to_string());
      }
      Err(_) => {}
    }
  }
  plugins
}

fn main() {
  // Create the system tray
  let quit_item = CustomMenuItem::new("quit".to_string(), "Quit");
  let hide_item = CustomMenuItem::new("hide".to_string(), "Hide");
  let devtools_item = CustomMenuItem::new("devtools".to_string(), "DevTools");
  let tray_menu = SystemTrayMenu::new()
    .add_item(hide_item)
    .add_item(devtools_item)
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(quit_item);
  let system_tray = SystemTray::new()
    .with_menu(tray_menu)
    // Don't show the menu on left clicks (only on macOS)
    .with_menu_on_left_click(false);

  tauri::Builder::default()
    // Configure system tray
    .system_tray(system_tray)
    .on_system_tray_event(|app, event| match event {
      // Left-click toggles window visibility
      SystemTrayEvent::LeftClick {
        position,
        size,
        ..
      } => {
        // Does the main window exist?
        match app.get_window("Functional") {
            Some(main_window) => {
              // Show or hide the main window
              if main_window.is_visible().unwrap() {
                main_window.hide().expect("Could not hide main window!")
              } else {
                main_window.show().expect("Could not show the main window!")
              }
            }
            None => {
              // Main window does not yet exist, so instantiate it
              let main_window = tauri::WindowBuilder::new(
                app,
                "Functional",
                tauri::WindowUrl::App("index.html".into())
              ).build().expect("Could not open main Window!");
              
              // Configure window
              main_window.set_title("Functional").expect("Could not set main window title!");
              main_window.set_always_on_top(true).expect("Could not set always on top to true");
              main_window.set_skip_taskbar(true).expect("Could not set skip taskbar");
              
              // Now to the sizing
              let win_width = 400.0;
              let win_x = position.x - win_width / 2.0 + size.width / 2.0;
              main_window.set_size::<LogicalSize<f64>>(LogicalSize::new(win_width, 600.0).into()).expect("Could not rezie main window");
              main_window.set_position(PhysicalPosition::new(win_x, position.y)).expect("Could not set window position");
            }
        }
      }
      SystemTrayEvent::MenuItemClick { id, .. } => {
          match id.as_str() {
            "quit" => {
              std::process::exit(0);
            }
            "hide" => {
              match app.get_window("Functional") {
                Some(main_window) => {
                  main_window.hide().expect("Could not hide main window!")
                }
                None => {}
              }
            }
            "devtools" => {
              match app.get_window("Functional") {
                Some(main_window) => {
                  if main_window.is_devtools_open() {
                    main_window.close_devtools()
                  } else {
                    main_window.open_devtools()
                  }
                }
                None => {}
              }
            }
            _ => {}
          }
      }
      _ => {}
    })
    // Build our event handlers
    .invoke_handler(tauri::generate_handler![list_plugins])
    // Start the app
    .build(tauri::generate_context!())
    .expect("Could not build application")
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
