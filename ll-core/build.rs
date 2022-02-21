use std::{fs, process::Command};

fn main() {
    // Git version
    let git_desc = Command::new("git")
        .args(&["describe", "--all", "--tags", "--dirty", "--long"])
        .output()
        .unwrap();

    fs::write("git.txt", String::from_utf8_lossy(&git_desc.stdout).trim()).unwrap();
}
