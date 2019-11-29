use std::process::Command;

pub fn generate_resources() {

    const COMMAND: &str = "glib-compile-resources";
    const INPUT: &str = "assets/resources.xml";
    const TARGET: &str = "resources.bin";

    let exists = Command::new("which").arg(COMMAND).output().unwrap();
    if !exists.status.success() {
        panic!(format!("Command '{}' not found", COMMAND));
    }

    let resources = Command::new(COMMAND)
    .args(&[INPUT, &format!("--target=out/{}", TARGET)])
    .output()
    .unwrap();

    if !resources.status.success() {
        panic!(format!("Failed to generate resources: {}", String::from_utf8_lossy(&resources.stderr)))
    }

}
