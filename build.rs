use std::env;
use cc;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    cc::Build::new()
        .file("src/add_one.c")
        .compile("add_one");


    println!("cargo::rustc-link-search=native={}", out_dir);
    println!("cargo::rustc-link-lib=static=add_one");
    println!("cargo::rerun-if-changed=src/add_one.c");
}
