#![windows_subsystem = "console"]
extern crate bindgen;
extern crate cmake;
extern crate path_clean;

use std::env;
use std::path::PathBuf;
use cmake::Config;
use path_clean::clean;


fn main() {
    // Run cmake on libicsneo
    let dst = Config::new("../../libicsneo")
        .build_target("icsneoc")
        .define("LIBICSNEO_BUILD_EXAMPLES", "OFF")
        .define("LIBICSNEO_BUILD_ICSNEOC_STATIC", "OFF")
        .define("LIBICSNEO_BUILD_ICSNEOLEGACY", "OFF")
        .build();
    // Debug output for lib path
    println!("cargo:warning=icsneoc.lib search path: {:?}", dst.join("build/Debug").display());
    println!("cargo:warning=icsneoc.lib search path: {:?}", dst.join("build/Release").display());
    // Configure the search path and lib name to link to
    println!("cargo:rustc-link-search=native={}", dst.join("build/Debug").display());
    println!("cargo:rustc-link-search=native={}", dst.join("build/Release").display());
    println!("cargo:rustc-link-lib=static=icsneoc");

    // lets generate the bindings
    let include_path = PathBuf::from(clean(format!("{}/../../include", env!("CARGO_MANIFEST_DIR")).as_str()));
    println!("cargo:warning=icsneo include path: {:?}", include_path);

    println!("cargo:warning={}", include_path.display());
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", include_path.display()))
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("cargo:warning=out_path: {:?}", out_path.display());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}