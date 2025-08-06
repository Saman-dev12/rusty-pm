use std::collections::HashSet;
use flate2::read::GzDecoder;
use reqwest::blocking::get;
use serde_json::Value;
use std::env;
use std::fs::{self, create_dir_all, File};
use std::io::{Cursor};
use tar::Archive;
use super::manifest::RustyManifest;

pub fn install(package: &str) {
    let current_dir = env::current_dir().expect("Failed to get current dir");
    let rusty_json_path = current_dir.join("package.json");

    if !rusty_json_path.exists() {
        println!("❌ package.json not found. Run `rpm init` first.");
        return;
    }

    let mut installed_packages = HashSet::new();

    let (latest_version, success) = install_recursive(package, &mut installed_packages);

    if success {
        let rusty_file_data = File::open(&rusty_json_path).expect("Failed to open package.json");
        let mut manifest: RustyManifest =
            serde_json::from_reader(rusty_file_data).expect("Failed to parse package.json");
        
        manifest.dependencies.push(format!("{} : {}", package, latest_version));
        manifest.dependencies.sort();
        manifest.dependencies.dedup();

        let manifest_file = File::create(&rusty_json_path).expect("Failed to rewrite package.json");
        serde_json::to_writer_pretty(manifest_file, &manifest)
            .expect("Failed to write updated manifest");

        println!("\n✅ Successfully installed {} and its dependencies.", package);
        println!("✅ Updated package.json");
    } else {
        println!("\n❌ Installation failed for package: {}", package);
    }
}


fn install_recursive(package_name: &str, installed: &mut HashSet<String>) -> (String, bool) {
    // 1. BASE CASE: If we've already installed this package, stop.
    if installed.contains(package_name) {
        return ("".to_string(), true); // Already installed, not an error.
    }
    println!("📦 Installing package: {}", package_name);

    let url = format!("https://registry.npmjs.org/{}", package_name);
    let response = match get(&url) {
        Ok(res) => res,
        Err(e) => {
            eprintln!("❌ Network error while fetching '{}': {}", package_name, e);
            return ("".to_string(), false);
        }
    };

    if !response.status().is_success() {
        eprintln!("❌ Could not find package '{}' on npm (Status: {}).", package_name, response.status());
        return ("".to_string(), false);
    }
    
    let text = response.text().expect("Failed to read response text.");
    let json: Value = serde_json::from_str(&text).expect("Failed to parse package JSON from registry.");

    let latest_version = json["dist-tags"]["latest"].as_str().expect("Latest version not found.").to_string();
    let tarball_url = json["versions"][&latest_version]["dist"]["tarball"].as_str().expect("Tarball URL not found.");

    println!("  -> Downloading {}@{}...", package_name, latest_version);
    let tarball_bytes = get(tarball_url).expect("Failed to download tarball.").bytes().expect("Failed to get tarball bytes.");
    let tar = GzDecoder::new(Cursor::new(tarball_bytes));
    let mut archive = Archive::new(tar);
    
    let current_dir = env::current_dir().expect("Failed to get current dir");
    let target_dir = current_dir.join("node_modules").join(package_name);
    create_dir_all(&target_dir).expect("Failed to create package directory.");
    archive.unpack(&target_dir).expect("Failed to extract package archive.");

    installed.insert(package_name.to_string());
    
    let package_subdir = target_dir.join("package");
    if package_subdir.is_dir() {
        for entry in fs::read_dir(&package_subdir).unwrap() {
            let entry = entry.unwrap();
            let source_path = entry.path();
            let dest_path = target_dir.join(entry.file_name());
            fs::rename(&source_path, &dest_path).expect("Failed to move content from 'package' subdir.");
        }
        fs::remove_dir(&package_subdir).expect("Failed to remove empty 'package' subdir.");
    }
    
    let package_json_path = target_dir.join("package.json");
    if package_json_path.exists() {
        let file = File::open(package_json_path).expect("Failed to open package.json");
        let pkg_json: Value = serde_json::from_reader(file).expect("Failed to parse package.json");

        if let Some(dependencies) = pkg_json["dependencies"].as_object() {
            println!("  -> Found {} dependencies for {}.", dependencies.len(), package_name);
            for (dep_name, _version) in dependencies {
                // RECURSIVE CALL
                let (_, success) = install_recursive(dep_name, installed);
                if !success {
                    eprintln!("  -> Failed to install dependency '{}' for '{}'", dep_name, package_name);
                    return ("".to_string(), false); // Propagate failure up
                }
            }
        }
    }

    (latest_version, true)
}
