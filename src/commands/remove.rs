use std::env;
use std::fs::{self, File};
use super::manifest::RustyManifest;


pub fn remove(package: &str) {
    let current_dir = env::current_dir().expect("Failed to get current dir");
    let rusty_json_path = current_dir.join("package.json");

    if !rusty_json_path.exists() {
        println!("❌ package.json not found. Run `rpm init` first.");
        return;
    }

    let rusty_file_data = File::open(&rusty_json_path).expect("Failed to open rusty.json");
    let mut manifest: RustyManifest =
        serde_json::from_reader(rusty_file_data).expect("Failed to parse rusty.json");

    manifest.dependencies.retain(|dep| !dep.trim().starts_with(package));

  let node_modules = current_dir.join("node_modules");
  let package_dir = node_modules.join(package);

  if package_dir.exists() {
    fs::remove_dir_all(&package_dir).expect("Failed to remove package directory");
    println!("🗑️ Removed {} from node_modules", package);
  } else {
    println!("⚠️ Package {} not found in node_modules", package);
  }

    let manifest_file = File::create(&rusty_json_path).expect("Failed to rewrite rusty.json");
    serde_json::to_writer_pretty(manifest_file, &manifest)
        .expect("Failed to write updated manifest");
}
