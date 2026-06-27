use ovmf_prebuilt::{Arch, FileType, Prebuilt, Source};
use std::process::Command;

fn main() {
    let uefi_path = env!("UEFI_PATH");

    let prebuilt = Prebuilt::fetch(Source::LATEST, "target/ovmf")
        .expect("failed to fetch OVMF firmware");
    let code = prebuilt.get_file(Arch::X64, FileType::Code);
    let vars = prebuilt.get_file(Arch::X64, FileType::Vars);

    let mut cmd = Command::new("qemu-system-x86_64");
    cmd.arg("-drive").arg(format!("format=raw,file={uefi_path}"));
    cmd.arg("-drive").arg(format!(
        "if=pflash,format=raw,unit=0,file={},readonly=on",
        code.display()
    ));
    cmd.arg("-drive").arg(format!(
        "if=pflash,format=raw,unit=1,file={},snapshot=on",
        vars.display()
    ));

    println!("UEFI image: {uefi_path}");

    let mut child = cmd.spawn().expect("failed to start qemu-system-x86_64");
    child.wait().expect("failed to wait on qemu");
}
