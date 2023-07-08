use std::process::Command;
use sysinfo::{System, SystemExt};

fn main() {
    include!(concat!(env!("OUT_DIR"), "/result.rs"));
}

/// Useful function
fn bash(cmd: &str) -> String {
    let output = Command::new("bash")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("Failed to execute command");

    String::from_utf8_lossy(&output.stdout).trim().to_string()
}
