use std::env;
use std::path::PathBuf;
use std::fs::File;
use std::io::Write;

fn main() {
    // Generate paths for UEFI and BIOS images
    let out_dir = env::var("OUT_DIR").unwrap();
    let uefi_path = PathBuf::from(&out_dir).join("uefi.img");
    let bios_path = PathBuf::from(&out_dir).join("bios.img");

    // Set environment variables
    println!("cargo:rustc-env=UEFI_PATH={}", uefi_path.display());
    println!("cargo:rustc-env=BIOS_PATH={}", bios_path.display());

    // Generate dummy UEFI and BIOS images for demonstration purposes
    let mut uefi_file = File::create(&uefi_path).unwrap();
    uefi_file.write_all(b"UEFI image content").unwrap();

    let mut bios_file = File::create(&bios_path).unwrap();
    bios_file.write_all(b"BIOS image content").unwrap();
   
        println!("cargo:rerun-if-changed=kernel_with_bootloader");
    
    
}