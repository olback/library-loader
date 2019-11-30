use std::{process::Command, path::Path, env, fs};

pub fn generate_rc() {

    fs::copy("assets/library-loader.rc", "out/library-loader.rc");

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
