use flate2::read::GzDecoder;
use reqwest::blocking::get;
use serde_json::Value;
use std::env;
use std::fs::{create_dir_all, File};
use std::io::{Cursor};
use tar::Archive;

use super::manifest::RustyManifest;


pub fn install(package: &str) {
    let current_dir = env::current_dir().expect("Failed to get current dir");
    let rusty_json_path = current_dir.join("package.json");

    if !rusty_json_path.exists() {
        println!("❌ rusty.json not found. Run `rusty-pm init` first.");
        return;
    }

    let rusty_file_data = File::open(&rusty_json_path).expect("Failed to open rusty.json");
    let mut manifest: RustyManifest =
        serde_json::from_reader(rusty_file_data).expect("Failed to parse rusty.json");

    let node_modules = current_dir.join("node_modules");
    if !node_modules.exists() {
        create_dir_all(&node_modules).expect("Failed to create node_modules folder");
    }

    let url = format!("https://registry.npmjs.org/{}", package);
    let response = get(&url)
        .expect("Failed to fetch package info")
        .text()
        .expect("Failed to read response");

    let json: Value = serde_json::from_str(&response).expect("Invalid JSON");

    let latest = json["dist-tags"]["latest"]
        .as_str()
        .expect("Latest version not found");

    let tarball_url = json["versions"][latest]["dist"]["tarball"]
        .as_str()
        .expect("Tarball URL not found");

    println!("📥 Downloading {}@{}...", package, latest);

    let tarball = get(tarball_url)
        .expect("Failed to download tarball")
        .bytes()
        .expect("Failed to get tarball bytes");

    let tar = GzDecoder::new(Cursor::new(tarball));
    let mut archive = Archive::new(tar);

    let target_dir = node_modules.join(package);
    create_dir_all(&target_dir).expect("Failed to create package folder");

    archive
        .unpack(&target_dir)
        .expect("Failed to extract package");

    println!(
        "✅ Installed {}@{} to {}",
        package,
        latest,
        target_dir.to_str().unwrap()
    );

	manifest.dependencies.push(format!("{} : {}",package.to_string(),latest.to_string()));  


    let manifest_file = File::create(&rusty_json_path).expect("Failed to rewrite rusty.json");
    serde_json::to_writer_pretty(manifest_file, &manifest)
        .expect("Failed to write updated manifest");
}

