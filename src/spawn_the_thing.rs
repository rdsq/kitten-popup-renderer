use std::process::Command;
use std::env::args;

pub fn spawn_the_thing(percent: u8, ansi_code: &str, time: &str) {
    let binary = args().nth(0).expect("failed to get binary name");
    Command::new("kitten")
        .args(["panel", &binary, "display", &percent.to_string(), ansi_code, time])
        .status()
        .expect("failed to run kitten");
}
