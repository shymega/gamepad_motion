#![allow(clippy::too_many_arguments)]

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::process::Command::new("git")
        .args([
            "submodule",
            "update",
            "--init",
            "--depth=1",
            "--recommend-shallow",
        ])
        .output()
        .expect("Failed to fetch git submodules!");
    let path = std::path::PathBuf::from("lib/GamepadMotionHelpers");
    let mut b = autocxx_build::Builder::new("src/lib.rs", [&path])
        .extra_clang_args(&["-std=c++17"])
        .build()?;
    b.flag_if_supported("-std=c++17")
        .flag_if_supported("-Ofast")
        .flag_if_supported("-Wno-comment")
        .flag_if_supported("-Wno-unused-parameter")
        .compile("gamepad_motion");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=lib/GamepadMotionHelpers/GamepadMotion.hpp");
    Ok(())
}
