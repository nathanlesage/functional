// This build script has the purpose of compiling the wrapper application for
// tauri (I presume some libraries/object files) that can then be linked against
// the actual app that we are writing in the src directory.

fn main() {
  tauri_build::build()
}
