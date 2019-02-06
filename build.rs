use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let profile = env::var("PROFILE").unwrap_or("Debug".to_string());
    let current_dir = std::env::current_dir().unwrap();
    let target;

    if profile == "Release" {
        target = Path::new(&current_dir).join("target/release");
    } else {
        target = Path::new(&current_dir).join("target/debug");
    }

    Command::new("rustc")
        .arg("examples/plugins/plugin_helloWorld.rs")
        .arg("--crate-name")
        .arg("dot_helloWorld")
        .arg("--crate-type")
        .arg("dylib")
        .arg("--out-dir")
        .arg(target)
        //.output()
        //.unwrap_or_else(|e| panic!("failed to execute process: {}", e))
        .status().unwrap_or_else(|e| panic!("failed to execute process: {}", e));
}
