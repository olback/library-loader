use std::{
    process::Command,
    fs
};
use regex;

fn generate_resources() {

    const COMMAND: &str = "glib-compile-resources";
    const INPUT: &str = "assets/resources.xml";
    const TARGET: &str = "resources.bin";

    let exists = match Command::new("which").arg(COMMAND).output() {
        Ok(v) => v,
        Err(e) => panic!("Error running command 'which:' {}", e)
    };
    if !exists.status.success() {
        panic!(format!("Command '{}' not found", COMMAND));
    }

    let resources = Command::new(COMMAND)
    .args(&[INPUT, &format!("--target={}", TARGET)])
    .output()
    .unwrap();

    if !resources.status.success() {
        panic!(format!("Failed to generate resources: {}", String::from_utf8_lossy(&resources.stderr)))
    }

}

fn fix_glade_resource_paths() {

    const GLADE_PATH: &str = "assets/library-loader.glade";

    let glade_xml_data = fs::read_to_string(GLADE_PATH).unwrap_or_else(|_| {
        panic!("{}#{}: Could not read file '{}'", std::file!(), std::line!(), GLADE_PATH);
    });
    let re = regex::Regex::new(r"(?P<r>resource:/)(?P<p>[a-z])").unwrap();
    let after = re.replace_all(&glade_xml_data, "$r//$p");

    fs::write(GLADE_PATH, after.to_owned().as_bytes()).unwrap();

}

fn main() {

    generate_resources();
    fix_glade_resource_paths();

}
