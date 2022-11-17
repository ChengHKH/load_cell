use std::env::consts;

fn main() {
    if consts::FAMILY == 'unix' {
        println!("cargo:rustc-env=CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER=../../msvc-linker/linker-x64.sh")
    }
}