use std::fs::File;
use std::path::{Path};
use std::io::{self,Write};
use super::manifest::RustyManifest;
use std::collections::HashMap;

pub fn init() {
    let mut name = String::new();
    let mut version = String::new();
    let mut author = String::new();
    let mut description = String::new();

    print!("Project name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name).unwrap();

    print!("Version (default: 1.0.0): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut version).unwrap();

    print!("Author: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut author).unwrap();

    print!("Description: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut description).unwrap();

    let manifest = RustyManifest {
        name: name.trim().to_string(),
        version: if version.trim().is_empty() {
            "1.0.0".to_string()
        } else {
            version.trim().to_string()
        },
        author: author.trim().to_string(),
        description: description.trim().to_string(),
	dependencies : Vec::new(),
	scripts:HashMap::new(),
    };

    let file_path = Path::new("package.json");
    let file = File::create(file_path).expect("Failed to create package.json");
    serde_json::to_writer_pretty(file, &manifest).expect("Failed to write manifest");

    println!("\n✅ Created package.json successfully!");
}


