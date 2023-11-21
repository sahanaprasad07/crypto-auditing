// SPDX-License-Identifier: GPL-2.0

use libbpf_cargo::SkeletonBuilder;
use std::{
    env,
    fs::{self, File},
    path::PathBuf,
    process::Command,
};

fn main() {
    let builddir =
        PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR must be set in build script"));
    let srcdir = PathBuf::from(
        env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR must be set in build script"),
    );

    let vmlinux_h = srcdir.join("src").join("bpf").join("vmlinux.h");
    if vmlinux_h.exists() {
        fs::copy(&vmlinux_h, &builddir.join("vmlinux.h")).expect("unable to copy vmlinux.h");
    } else {
        let file = File::create(&builddir.join("vmlinux.h")).expect("unable to create vmlinux.h");
        Command::new("bpftool")
            .arg("btf")
            .arg("dump")
            .arg("file")
            .arg("/sys/kernel/btf/vmlinux")
            .arg("format")
            .arg("c")
            .stdout(file)
            .status()
            .expect("unable to run bpftool");
    }
    let src = srcdir.join("src").join("bpf").join("audit.bpf.c");
    SkeletonBuilder::new()
        .source(&src)
        .clang_args(&format!("-I{}", builddir.display()))
        .build_and_generate(&builddir.join("audit.skel.rs"))
        .unwrap();
    println!("cargo:rerun-if-changed={}", src.display());
}
