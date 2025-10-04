use std::process::Command;

pub fn get_percent() -> u8 {
    let max_b = Command::new("brightnessctl")
        .arg("max")
        .output()
        .expect("failed to call brightnessctl")
        .stdout;
    let max_str = String::from_utf8(max_b).expect("invalid utf8 from `brightnessctl max`");
    let max_num: f32 = max_str.trim_end().parse().expect("`brightnessctl max` unexpected output");
    let now_b = Command::new("brightnessctl")
        .arg("get")
        .output()
        .expect("failed to call brightnessctl (again? how?)")
        .stdout;
    let now_str = String::from_utf8(now_b).expect("invalid utf8 from `brightnessctl get`");
    let now_num: f32 = now_str.trim_end().parse().expect("`brightnessctl get` unexpected output");
    let fraction = now_num / max_num;
    let percent = fraction * 100.0;
    return percent.round() as u8;
}
