use cmake::{self, Config};
use std::path::PathBuf;

fn main() {
    // Build minizip with cmake
    let dst = Config::new("minizip")
        .define("MZ_BZIP2_STATIC", "ON")
        .define("MZ_ZLIB_STATIC", "ON")
        .target("minizip zlibstatic")
        .build();

    // Tell cargo to tell rustc to link the minizip library.
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );

    println!("cargo:rustc-link-lib=static=minizip");

    let target = std::env::var("TARGET").unwrap();
    if target.contains("apple") {
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=Security");
        println!("cargo:rustc-link-lib=static=z");
        println!("cargo:rustc-link-lib=iconv");
    }
    if target.contains("windows") {
        println!("cargo:rustc-link-lib=static=zlibstatic");
    }

    // Generate Rust FFI bindings
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate_comments(true)
        .use_core()
        .ctypes_prefix("libc")
        .whitelist_function("mz_.*") // it adds recursively all used types so the next line in this case changes nothing for this particular case
        .whitelist_type("mz_.*")
        .prepend_enum_name(false)
        .constified_enum_module("mz_op")
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to src folder to make rls autocomplete work.
    let out_path = PathBuf::from("src");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
