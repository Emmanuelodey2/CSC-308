fn main() {
    // Read env variables that were set in the build script
    let uefi_path: &str = &std::env::var("UEFI_PATH").expect("UEFI_PATH not set");
    let bios_path: &str = &std::env::var("BIOS_PATH").expect("BIOS_PATH not set");

    // Print paths for debugging
    println!("UEFI_PATH is: {}", uefi_path);
    println!("BIOS_PATH is: {}", bios_path);

    // Choose whether to start the UEFI or BIOS image
    let uefi: bool = false;
    let mut cmd = std::process::Command::new("qemu-system-x86_64");
    if uefi {
        cmd.arg("-bios").arg(ovmf_prebuilt::ovmf_pure_efi());
        cmd.arg("-drive").arg(format!("format=raw,file={}", uefi_path));
    } else {
        cmd.arg("-drive").arg(format!("format=raw,file={}", bios_path));
    }

    let mut child = cmd.spawn().expect("Failed to start QEMU");
    child.wait().expect("Failed to wait on child process");
}