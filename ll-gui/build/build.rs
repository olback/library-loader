mod glade;
mod out;
mod resources;
mod windows;

use std::env;

fn main() {
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=assets/*");

    out::output_dir();
    glade::fix_resource_paths();
    resources::generate_resources();

    if env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        windows::generate_rc();
        windows::compile_rc();
    }
}
