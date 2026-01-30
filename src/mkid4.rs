// mkid4 - Quick UUID v4 generator
// Thin wrapper that calls: mkid uuid v4 [args...]

use std::env;
use std::process::{Command, exit};

fn main() {
    // Get the directory where this binary is located
    let current_exe = env::current_exe().expect("Failed to get current executable path");
    let bin_dir = current_exe
        .parent()
        .expect("Failed to get binary directory");
    let mkid_path = bin_dir.join("mkid");

    // Collect all arguments passed to mkid4
    let args: Vec<String> = env::args().skip(1).collect();

    // Execute: mkid uuid v4 [args...]
    let status = Command::new(mkid_path)
        .arg("uuid")
        .arg("v4")
        .args(&args)
        .status()
        .expect("Failed to execute mkid");

    exit(status.code().unwrap_or(1));
}
