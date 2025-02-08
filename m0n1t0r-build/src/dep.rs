use std::process::Command;

pub fn check_xmake() {
    Command::new("xmake")
        .arg("--help")
        .output()
        .expect("No xmake found. Please install xmake.");
}
