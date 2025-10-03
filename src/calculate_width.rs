use terminal_size::{terminal_size, Width};

fn decimal_width(num: &u8) -> u16 {
    if *num >= 100 {
        3
    } else if *num >= 10 {
        2
    } else {
        1
    }
}

pub fn calculate_width(percent: u8) -> (u16, u16) {
    let percent_length = decimal_width(&percent) + 4; // including `% ` and `[` `]`
    let (Width(width), _) = terminal_size().expect("failed to get terminal size");
    let available_width = width - percent_length;
    let fl = (available_width as f32) * ((percent as f32) / 100.0);
    return (fl.round() as u16, available_width);
}
