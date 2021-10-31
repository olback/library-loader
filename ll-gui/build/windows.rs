use std::{process::Command, path::Path, env, fs};
use toml;
use serde::Deserialize;

#[derive(Deserialize)]
struct CargoTomlPackage {
    version: String
}

#[derive(Deserialize)]
struct CargoToml {
    package: CargoTomlPackage
}

pub fn generate_rc() {

    const RC_IN: &str = "assets/library-loader.rc";
    const RC_OUT: &str = "out/library-loader.rc";

    const CARGO_TOML: &str = include_str!("../Cargo.toml");
    let ct: CargoToml = match toml::from_str(CARGO_TOML) {
        Ok(v) => v,
        Err(e) => panic!("{}#{}: Error parsing Cargo.toml: {}", std::file!(), std::line!(), e)
    };

    let rc_content = fs::read_to_string(RC_IN).unwrap();
    let new_rc_content = rc_content.replace("<VERSION_STRING>", &ct.package.version);
    fs::write(RC_OUT, new_rc_content).unwrap();

}

pub fn compile_rc() {

    let out_dir = env::var("OUT_DIR").unwrap();
    match Command::new("x86_64-w64-mingw32-windres")
    .args(&["out/library-loader.rc", &format!("{}/program.o", out_dir)])
    .status() {
        Ok(s) => {
            if !s.success() {
                panic!("{}#{}: x86_64-w64-mingw32-windres failed", std::file!(), std::line!())
            }
        },
        Err(e) => {
            panic!("{}#{}: x86_64-w64-mingw32-windres failed {}", std::file!(), std::line!(), e)
        }
    };

    match Command::new("x86_64-w64-mingw32-gcc-ar")
    .args(&["crus", "libprogram.a", "program.o"])
    .current_dir(&Path::new(&out_dir))
    .status() {
        Ok(s) => {
            if !s.success() {
                panic!("{}#{}: x86_64-w64-mingw32-gcc-ar failed", std::file!(), std::line!())
            }
        },
        Err(e) => {
            panic!("{}#{}: x86_64-w64-mingw32-gcc-ar failed {}", std::file!(), std::line!(), e)
        }
    };

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=program");

}
