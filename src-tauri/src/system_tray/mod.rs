use tauri::{
  SystemTray,
  SystemTrayEvent,
  SystemTrayMenu,
  SystemTrayMenuItem,
  CustomMenuItem,
  Manager,
  LogicalSize,
  PhysicalPosition,
  AppHandle, PhysicalSize
};

// Shows or hides the main window
fn show_or_hide_main_window (
  app: &AppHandle,
  tray_position: PhysicalPosition<f64>,
  tray_size: PhysicalSize<f64>
) -> () {
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
      
      // Now to the sizing. We need to position the window centered below the
      // tray item on instantiation. Afterwards, the user may drag it to
      // wherever they want.
      let win_width = 400.0;
      let win_x = tray_position.x - win_width / 2.0 + tray_size.width / 2.0;

      main_window.set_size::<LogicalSize<f64>>(LogicalSize::new(win_width, 600.0).into())
        .expect("Could not rezie main window");
      main_window.set_position(PhysicalPosition::new(win_x, tray_position.y))
        .expect("Could not set window position");
    }
  }
}

// Create the system tray
pub fn create_system_tray () -> SystemTray {
  let quit_item = CustomMenuItem::new("quit".to_string(), "Quit");
  let hide_item = CustomMenuItem::new("hide".to_string(), "Hide");
  let devtools_item = CustomMenuItem::new("devtools".to_string(), "DevTools");

  // Now, create the tray menu
  let tray_menu = SystemTrayMenu::new()
    .add_item(hide_item)
    .add_item(devtools_item)
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(quit_item);

  // And then build the tray itself & return it
  SystemTray::new()
    .with_menu(tray_menu)
    // Don't show the menu on left clicks (only on macOS)
    .with_menu_on_left_click(false)
}

// Handles system tray events
pub fn handle_system_tray_events (app: &AppHandle, event: SystemTrayEvent) -> () {
  match event {
    // Left-click toggles window visibility
    SystemTrayEvent::LeftClick {
      position,
      size,
      ..
    } => {
      show_or_hide_main_window(app, position, size);
    }
    SystemTrayEvent::MenuItemClick { id, .. } => {
        match id.as_str() {
          "quit" => {
            std::process::exit(0);
          }
          "hide" => {
            match app.get_window("Functional") {
              Some(main_window) => {
                if main_window.is_visible().unwrap() {
                  main_window.hide().expect("Could not hide main window!")
                }
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
  }
}
