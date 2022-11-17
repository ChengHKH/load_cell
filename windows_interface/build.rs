use std::env::consts;

#[cfg(target_env = "msvc")]
fn main() {
    if consts::OS == "linux" {
        println!("cargo:rustc-link-arg-bin=windows_interface=../../msvc-linker/linker-x64.sh")
    }
}