mod out;
mod resources;
mod glade;
mod windows;

use std::env;

fn main() {

    println!("cargo:rerun-if-changed=.git/HEAD");

    out::output_dir();
    resources::generate_resources();
    glade::fix_resource_paths();

    if env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        windows::generate_rc();
        windows::compile_rc();
    }

}
