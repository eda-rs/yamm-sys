extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::{process::Command, str};

fn main(){
    println!("cargo:rustc-link-lib=llvm");
    println!("cargo:rerun-if-changed=yamm.h");

    let llvm_config_out = Command::new("llvm-config")
        .args(&["--cxxflags", "--ldflags", "--system-libs", "--libs", "core"])
        .output()
        .expect("failed to execute llvm-config");

    let llvm_clang_args = llvm_config_out
        .stdout
        .split(|byte| byte.is_ascii_whitespace())
        .map(|arg| str::from_utf8(arg).unwrap());

    let bindings = bindgen::Builder::default()
        .header("yamm/src/cpp/yamm.h")
        // .allowlist_type("yamm_ns_yamm_buffer")
        .clang_arg("-x").clang_arg("c++") // c++ flag
        .clang_arg("-std=c++11")
        .clang_args(llvm_clang_args)
        .allowlist_type("^yamm_.*").allowlist_recursively(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("couldn't write bindings!");
}