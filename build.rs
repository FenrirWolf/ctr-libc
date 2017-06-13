extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let dka_path = env::var("DEVKITARM").unwrap();

    let bindings = bindgen::Builder::default()
        .use_core()
        .trust_clang_mangling(false)
        .generate_comments(false)
        .layout_tests(false)
        .ctypes_prefix("")
        .header("wrapper.h")
        .clang_arg(format!("--sysroot={}arm-none-eabi/", dka_path))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}
