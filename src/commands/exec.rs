use std::process::Command;
use std::io;
use std::io::Write;


pub fn exec_command(command_string: &str) {
    // This part sets up the command for Windows or macOS/Linux and is correct.
    let mut command = if cfg!(target_os = "windows") {
        let mut cmd = Command::new("cmd");
        cmd.arg("/C");
        cmd
    } else {
        let mut cmd = Command::new("sh");
        cmd.arg("-c");
        cmd
    };
    
    command.arg(command_string);

    let status = command.status().expect("Failed to execute command");

    // After the command finishes (e.g., by you pressing Ctrl+C),
    // you can check if it exited successfully.
    if !status.success() {
        // This message will appear after you stop the server if it crashed.
        eprintln!("\nScript exited with an error.");
    }
}
