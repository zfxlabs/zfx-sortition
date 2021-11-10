#[cfg(not(windows))]
extern crate cc;
#[cfg(target_env = "msvc")]
extern crate libc;

extern crate pkg_config;
extern crate walkdir;

/*
fn main() {
    println!("cargo:rustc-link-lib=foo");
}*/

// COMMENTS FROM G SOURCE
// #cgo CFLAGS: -O3
// #cgo CXXFLAGS: -std=c++11
// #include <stdint.h>
// #include <stdlib.h>
// #include "sortition.h"

fn main() {
    cxx_build::bridge("src/lib.rs")
        .cpp(true)
        .file("src/sortition.cpp")
        .flag_if_supported("-std=c++14")
        .flag_if_supported("-fPIC")
        .compile("zfx-sortition-bridge");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/sortition.cpp");
    println!("cargo:rerun-if-changed=src/sortition.h");

    /*
    println!("cargo:rustc-link-lib=foo");
    pkg_config::Config::new()
        .atleast_version("1.2")
        .probe("zfx-sortition")
        .unwrap();
     */

    /*let src = ["src/sortition.h", "src/sortition.cpp"];
    let mut builder = cc::Build::new();
    let build = builder
        .files(src.iter())
        .include("include")
        .flag("-O3")
        .flag("-std=c++11")
        .flag("-Wno-unused-parameter");
    //.define("USE_ZLIB", None);
    build.compile("foo");
     */
}
