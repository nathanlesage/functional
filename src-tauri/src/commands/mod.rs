use std::path::Path;
use std::env;
use std::fs::{read_dir, metadata, create_dir};

mod plugins;
use plugins::parse_plugin;

use self::plugins::PluginDescriptor;

/** Retrieves the data directory (user data in prod, otherwise ./plugins) */
#[cfg(debug_assertions)]
pub fn retrieve_data_dir() -> String {
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
pub fn retrieve_data_dir() -> String {
  let mut data_dir = tauri::api::path::data_dir().unwrap();
  data_dir.push("com.zettlr.functional");
  data_dir.to_str().unwrap().to_string()
}

#[tauri::command]
pub fn list_plugins() -> Vec<PluginDescriptor> {
  let data_dir = retrieve_data_dir();
  let mut plugins: Vec<PluginDescriptor> = Vec::new();

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
        if entry.metadata().unwrap().is_dir() {
          let plugin_path = entry.path().to_str().unwrap().to_string();
          let result = parse_plugin(plugin_path);
          match result {
            Ok(descriptor) => {
              plugins.push(descriptor)
            }
            Err(_e) => {}
          }
        }
      }
      Err(_) => {}
    }
  }
  plugins
}

// Returns the full path to the plugin.js file, given a plugin path
#[tauri::command]
pub fn get_plugin_js(plugin_path: String) -> String {
  let mut path = Path::new(&plugin_path);
  let buf = path.join("plugin.js");
  path = buf.as_path();

  path.to_str().unwrap().to_string()
}
