//! Source: https://docs.rs/riscv-rt/0.8.0/riscv_rt/index.html

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Copies the memory.x file into the out directory so that the linker script can find it.
fn main() {
    let out_dir = env::var("OUT_DIR").expect("No out dir");
    let dest_path = Path::new(&out_dir);
    let mut f = File::create(&dest_path.join("memory.x"))
        .expect("Could not create file");
    f.write_all(include_bytes!("memory.x"))
        .expect("Could not write file");
    println!("cargo:rustc-link-search={}", dest_path.display());
    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rerun-if-changed=build.rs");
}
