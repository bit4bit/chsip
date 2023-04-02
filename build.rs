extern crate bindgen;

use std::collections::HashSet;
use std::env;
use std::path::PathBuf;

fn main() {
    let ignored_macros = IgnoreMacros(
            vec![
                "IPPORT_RESERVED".into(),
            ]
            .into_iter()
            .collect(),
        );

    // DERIVADO DE: https://rust-lang.github.io/rust-bindgen/tutorial-3.html

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=sofia-sip-ua");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        .clang_arg("-I/usr/include/sofia-sip-1.12/")
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .opaque_type("msg_hclass.*")
        //ISSUE: https://github.com/rust-lang/rust-bindgen/issues/687
        //`IPPORT_RESERVED` redefined here
        .parse_callbacks(Box::new(ignored_macros))
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
