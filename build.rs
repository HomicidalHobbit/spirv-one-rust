use std::env;
use std::path::Path;

fn main() {
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-lib=dylib=c++");
    println!(
        "cargo:rustc-link-search=native={}",
        Path::new(&dir).display()
    );
}