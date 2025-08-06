use std::env;
use super::manifest::RustyManifest;
use super::exec::exec_command;
use std::fs::File;

pub fn run_script(script_name: &str) {
    let current_dir = env::current_dir().expect("Failed to get current dir");
    let rusty_json_path = current_dir.join("package.json");

    if !rusty_json_path.exists() {
        println!("❌ rusty.json not found. Run `rusty-pm init` first.");
        return;
    }

    let rusty_file_data = File::open(&rusty_json_path).expect("Failed to open package.json");
    let manifest: RustyManifest =
        serde_json::from_reader(rusty_file_data).expect("Failed to parse package.json");

    if let Some(script) = manifest.scripts.get(script_name) {
        println!("Executing script: {}", script);
        exec_command(script);
    } else {
        println!("Script '{}' not found in rusty.json", script_name);
    }
}
