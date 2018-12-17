use std::path::PathBuf;
use std::fs;
use std::io::prelude::*;
use std::env;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=WINDIVERT_DYNAMIC");

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

    // copy over all the includes
    fs::create_dir_all(include.join("windivert")).unwrap();
    fs::copy("", include.join("windivert/");

    // we only need to build the DLL portion

    std::env::set("CFLAGS", "-shared -O2 -Iinclude -Wl,--enable-stdcall-fixup -Wl,--entry=_WinDivertDllEntry")

    cc::Build::new()
        .file("./bundled/src/dll/windivert.c")
        .file("./bundled/src/dll/windivert_helper.c")
        .compile("WinDivert")

    // set_link_directives();
}

fn set_link_directives() {
    let kind = if env::var("WINDIVERT_DYNAMIC").is_ok() {
        "dylib"
    } else {
        "static"
    }

    println!("cargo:rustc-link-lib={}=WinDivert", kind);
}