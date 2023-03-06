extern crate bindgen;
extern crate cc;

use std::path::PathBuf;
use std::{env, fs};

fn main() {
    let mut cfg = cc::Build::new();
    cfg.warnings(false);

    let dst = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    cfg.include("scrypt-jane/code")
        .include("scrypt-jane")
        .define("SCRYPT_CHACHA", None)
        .define("SCRYPT_KECCAK512", None)
        .files(["scrypt-jane/scrypt-jane.c"])
        .out_dir(dst.join("lib"))
        .compile("libscryptjane.a");

    let src = env::current_dir().unwrap().join("scrypt-jane");
    let include = dst.join("include");
    fs::create_dir_all(&include).unwrap();
    fs::copy(src.join("scrypt-jane.h"), dst.join("include/scrypt-jane.h")).unwrap();
    println!("cargo:root={}", dst.display());
    println!("cargo:include={}", dst.join("include").display());

    println!("cargo:rerun-if-changed=wrapper.h");
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .allowlist_function("scrypt")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
