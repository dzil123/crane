extern crate bindgen;

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    fs::remove_dir_all(&out_path).unwrap();

    let mut go_build = Command::new("go");
    go_build
        .current_dir("./go")
        .env("CGO_ENABLED", "1")
        .env("GOOS", "linux")
        .env("GOARCH", "amd64")
        .arg("build")
        .arg("-buildmode=c-archive")
        .arg("-trimpath")
        .arg("-o")
        .arg(out_path.join("libgo.a"))
        .arg(".");

    let exit_status = go_build.status().expect("Go build failed");
    if !exit_status.success() {
        panic!("Failed to run `{go_build:?}`: {exit_status:?}");
    }

    let bindings = bindgen::Builder::default()
        .header(out_path.join("libgo.h").to_str().unwrap())
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .allowlist_function("GetBuildInfo")
        .allowlist_function("ImageMetadata")
        .allowlist_function("FreeImageMetadataReturn")
        .allowlist_function("FreeStr")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=Cargo.lock");
    println!("cargo:rerun-if-changed=go/go.mod");
    println!("cargo:rerun-if-changed=go/go.sum");
    println!("cargo:rerun-if-changed=go/lib.go");
    println!(
        "cargo:rustc-link-search=native={}",
        out_path.to_str().unwrap()
    );
    println!("cargo:rustc-link-lib=static=go");
}
