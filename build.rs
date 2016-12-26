// build.rs
// Copyright (C) 2016 jinpengzhang <jinpengzhang@jinpengdeMacBook-Pro.local>
// Distributed under terms of the MIT license.
//

extern crate libbindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell Cargo to tell rustc to link the system rocksdb shared library.
    println!("cargo:rustc-link-lib=rocksdb");

    let bindings = libbindgen::Builder::default()
        .no_unstable_rust()
        .clang_arg("-std=c++11")
        .header("wrapper.hpp")
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs")).expect("Couldn't write bindings!");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
