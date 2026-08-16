use std::{env::var, process::Command};

fn main() {
    let target = var("CARGO_TARGET_DIR").unwrap_or("target".to_string());
    let profile = var("PROFILE").unwrap();
    let cc =  var("CC").unwrap_or("cc".to_string());
    Command::new(cc)
        .args([
            "-o", format!("{}/{}/noncrust", target, profile).as_str(),
            "src/compare.c"
        ])
        .spawn().unwrap()
        .wait().unwrap();
}
