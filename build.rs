use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=kernel/src");
    println!("cargo:rerun-if-changed=kernel/Cargo.toml");

    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let kernel_manifest = manifest_dir.join("kernel/Cargo.toml");
    let cargo = env::var_os("CARGO").unwrap_or_else(|| "cargo".into());

    let status = Command::new(&cargo)
        .arg("build")
        .arg("--release")
        .arg("--target")
        .arg("x86_64-unknown-none")
        .arg("--manifest-path")
        .arg(&kernel_manifest)
        .status()
        .expect("failed to spawn cargo for kernel build");

    if !status.success() {
        panic!("kernel build failed");
    }

    let kernel_elf = manifest_dir.join("kernel/target/x86_64-unknown-none/release/kernel");

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let uefi_path = out_dir.join("dogos-uefi.img");
    bootloader::UefiBoot::new(&kernel_elf)
        .create_disk_image(&uefi_path)
        .unwrap();

    println!("cargo:rustc-env=UEFI_PATH={}", uefi_path.display());
}
