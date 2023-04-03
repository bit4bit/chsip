extern crate bindgen;

use std::collections::HashSet;
use std::env;
use std::path::{PathBuf, Path};

use bindgen::CargoCallbacks;

// DERIVADO DE: https://rust-lang.github.io/rust-bindgen/tutorial-3.html
fn main() {
    
    let sofia_library_path = Path::new("/usr/include/sofia-sip-1.12");

    let libdir_path = PathBuf::from("sofia_app")
        .canonicalize()
        .expect("cannot canonicalize path");
    let headers_path = libdir_path.join("sofia_app.h");

    println!("cargo:rustc-link-search={}", libdir_path.to_str().unwrap());
    println!("cargo:rustc-link-lib=static=ssl");
    println!("cargo:rustc-link-lib=static=crypto");
    //we have libsofia-sip-ua.a
    //without static= not works
    println!("cargo:rustc-link-lib=static=sofia-sip-ua");
    println!("cargo:rustc-link-lib=static=sofia_app");
    println!("cargo:rustc-link-arg-bins=-lsofia-sip-ua");
    println!("cargo:rustc-link-arg-bins=-lssl");
    println!("cargo:rustc-link-arg-bins=-lcrypto");
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed={}", "sofia_app/sofia_app.h");
    println!("cargo:rerun-if-changed={}", "sofia_app/sofia_app.c");

    cc::Build::new()
        .file("sofia_app/sofia_app.c")
        .include(sofia_library_path)
        .static_flag(true)
        .compile("libsofia_app.a");

    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(headers_path.to_str().unwrap())
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}


#[derive(Debug)]
struct IgnoreMacros(HashSet<String>);

impl bindgen::callbacks::ParseCallbacks for IgnoreMacros {
    fn will_parse_macro(&self, name: &str) -> bindgen::callbacks::MacroParsingBehavior {
        if self.0.contains(name) {
            bindgen::callbacks::MacroParsingBehavior::Ignore
        } else {
            bindgen::callbacks::MacroParsingBehavior::Default
        }
    }
}
