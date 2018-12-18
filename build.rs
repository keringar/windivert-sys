use std::path::PathBuf;
use std::env;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let target = env::var("TARGET").unwrap();
    let target: Vec<_> = target.split('-').collect();
    if target.get(2) == Some(&"windows") {
        build();
    }
}

fn build() {
    let dst = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let include = dst.join("include");
    let build = dst.join("build");
    println!("cargo:root={}", dst.display());
    println!("cargo:include={}", include.display());
    println!("cargo:static=1");

    // we only need to build the DLL portion
    let mut cfg = cc::Build::new();
    cfg.out_dir(&build)
        .include("vendored/divert/include")
        .include("vendored/divert/dll/")
        .file("vendored/divert/dll/windivert.c")
        .warnings(false);

    cfg.compile("windivert");

    set_link_directives();
}

fn set_link_directives() {
    // required by windivert
    println!("cargo:rustc-link-lib=advapi32");
    println!("cargo:rustc-link-lib=setupapi");
    println!("cargo:rustc-link-lib=user32");
    println!("cargo:rustc-link-lib=kernel32");
    println!("cargo:rustc-link-lib=ws2_32");

    // statically link the vendored windivert
    println!("cargo:rustc-link-lib=static=windivert");
}