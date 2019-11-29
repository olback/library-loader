pub fn generate_rc() {

}

pub fn compile_rc() {

    let out_dir = env::var("OUT_DIR").unwrap();
    Command::new("x86_64-w64-mingw32-windres")
    // .args(&["src/program.rc"])
    // .arg(&format!("{}/program.o", out_dir))
    .args(&["out/library-loader.rc", &format!("{}/program.o", out_dir)])
    .status().unwrap();

    Command::new("x86_64-w64-mingw32-gcc-ar")
    .args(&["crus", "libprogram.a", "program.o"])
    .current_dir(&Path::new(&out_dir))
    .status().unwrap();

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=program");

}
