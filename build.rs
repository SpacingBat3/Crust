use std::{env::var, os::{raw::{c_char, c_double}, unix::process::CommandExt}, process::Command};

fn main() {
    let target = var("CARGO_TARGET_DIR").unwrap_or("target".to_string());
    let profile = var("PROFILE").unwrap();
    let cc =  var("CC").unwrap_or("cc".to_string());
    let _ = Command::new(cc)
        .args([
            "-o", format!("{}/{}/noncrust", target, profile).as_str(),
            "src/compare.c"
        ])
        .exec();
}
