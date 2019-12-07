use std::env;
use std::path::Path;

fn main() {
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let libs_path = set_libs();
    println!(
        "cargo:rustc-link-search=native={}",
        Path::new(&dir).join(libs_path).display()
    );
}

#[cfg(target_os = "macos")]
fn set_libs() -> String {
    println!("cargo:rustc-link-lib=dylib=c++");
    //println!("cargo:rustc-link-lib=dylib=iconv");
    String::from("macos")
}
#[cfg(target_os = "linux")]
fn set_libs() -> String {
    println!("cargo:rustc-link-lib=static=stdc++");
    String::from("linux")
}
#[cfg(target_os = "windows")]
fn set_libs() -> String {
    String::from("windows")
}
