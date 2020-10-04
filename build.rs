use std::env;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    if !(Command::new("nasm")
        .args(&[
            "-o",
            &(out_dir.clone() + "/a.o"),
            "-g",
            "-F",
            "dwarf",
            "-Werror",
            "-f",
            "elf64",
            "qsort.asm",
        ])
        .status()
        .unwrap()
        .success()
        && Command::new("ar")
            .args(&[
                "-crus",
                &(out_dir.clone() + "/liba.a"),
                &(out_dir.clone() + "/a.o"),
            ])
            .status()
            .unwrap()
            .success())
    {
        panic!("failed");
    }
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=a");
}
