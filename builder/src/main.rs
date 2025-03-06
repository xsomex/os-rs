use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from("../bootable/");
    let kernel = PathBuf::from("../kernel/target/x86_64-unknown-none/debug/kernel");

    let uefi_path = out_dir.join("uefi.img");
    bootloader::UefiBoot::new(&kernel)
        .create_disk_image(&uefi_path)
        .unwrap();

    let bios_path = out_dir.join("bios.img");
    bootloader::BiosBoot::new(&kernel)
        .create_disk_image(&bios_path)
        .unwrap();
    
    let uefi = true;

    let mut cmd = std::process::Command::new("qemu-system-x86_64");
    if uefi {
        cmd.arg("-bios").arg(ovmf_prebuilt::ovmf_pure_efi());
        cmd.arg("-drive")
            .arg(format!("format=raw,file={}", uefi_path.display()));
    } else {
        cmd.arg("-drive")
            .arg(format!("format=raw,file={}", bios_path.display()));
    }
    let mut child = cmd.spawn().unwrap();
    child.wait().unwrap();
}
