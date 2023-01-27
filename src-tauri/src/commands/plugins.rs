use std::fs::{metadata, read_dir, read_to_string};
use std::path::Path;
use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};
use serde_yaml;

struct PluginMetadata {
  yaml_path: String
}

fn verify_plugin (plugin_path: &String) -> Result<PluginMetadata, &'static str> {
  // Our plugins must ...
  // * ... be a directory
  // * ... that contains a plugin.js file
  // * ... that contains an plugin.yml file
  // * ... that contains the required fields
  match metadata(&plugin_path) {
    Ok(metadata) => {
      if !metadata.is_dir() {
        return Err("Plugin Path is not a directory")
      }
    }
    Err(_e) => {
      return Err("Plugin Path is not a directory")
    }
  }

  // It's a directory, so that's alright. Now check for the required files.
  let mut plugin_js = false;
  let mut plugin_yaml = false;
  let mut is_yml = false;

  for entry_res in read_dir(plugin_path).unwrap() {
    match entry_res {
      Ok(entry) => {
        let metadata = entry.metadata().unwrap();
        let file_name = entry.file_name();
        if !metadata.is_file() {
          continue
        }

        if file_name.to_str().unwrap() == "plugin.yaml" {
          plugin_yaml = true;
        } else if file_name.to_str().unwrap() == "plugin.yml" {
          plugin_yaml = true;
          is_yml = true;
        } else if file_name.to_str().unwrap() == "plugin.js" {
          plugin_js = true;
        }
      }
      Err(_) => {}
    }
  }

  if !plugin_js {
    return Err("No plugin.js found")
  }
  if !plugin_yaml {
    return Err("No plugin.yaml found")
  }

  // Finally, set the yaml path
  let mut yaml_path = Path::new(&plugin_path).to_path_buf();
  if is_yml {
    yaml_path = yaml_path.join("plugin.yml");
  } else {
    yaml_path = yaml_path.join("plugin.yaml");
  }

  Ok(PluginMetadata { yaml_path: yaml_path.as_path().to_str().unwrap().to_string() })
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PluginDescriptor {
  path: String,
  name: String,
  description: String,
  version: String,
  author: String
}

impl PluginDescriptor {
  pub fn new(path: String, name: String, description: String, version: String, author: String) -> Self {
    Self { path, name, description, version, author }
  }
}

pub fn parse_plugin (plugin_path: String) -> Result<PluginDescriptor, &'static str> {
  match verify_plugin(&plugin_path) {
    Err(e) => {
      return Err(e)
    }
    Ok(metadata) => {
      // Plugin seems valid, so now we can parse it
    let contents = read_to_string(metadata.yaml_path).unwrap();
    let result = serde_yaml::from_str::<BTreeMap<String, String>>(&contents);
    match result {
      Err(_e) => {
        return Err("Could not read plugin.yaml.")
      }
      Ok(yaml) => {
        return Ok(
          PluginDescriptor::new(
            plugin_path,
            yaml.get("name").unwrap().to_string(),
            yaml.get("description").unwrap().to_string(),
            yaml.get("version").unwrap().to_string(),
            yaml.get("author").unwrap().to_string()
          )
        )
      }
    }
    }
  }
}
