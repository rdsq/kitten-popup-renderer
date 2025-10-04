use regex::Regex;
use std::process::Command;

fn parse_amixer_output(output: &str) -> (u8, bool) {
    let re = Regex::new(r"\[(\d+)%\] \[(on|off)\]").unwrap();

    if let Some(caps) = re.captures(output) {
        let percent = caps[1].parse::<u8>().unwrap_or(0);
        let mute_status = &caps[2] == "off";
        return (percent, mute_status);
    }

    panic!("failed to parse amixer output");
}

pub fn get_volume() -> (u8, bool) {
    let output = Command::new("amixer")
        .args(["sget", "Master"])
        .output()
        .expect("failed to call amixer")
        .stdout;
    let output_str = String::from_utf8(output).expect("failed to parse amixer output as utf8");
    parse_amixer_output(&output_str)
}
