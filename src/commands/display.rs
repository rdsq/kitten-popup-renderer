use crate::calculate_width::calculate_width;
use std::io::{Write, self};
use std::{thread, time};

#[derive(clap::Parser, Debug)]
/// Internal command to create the contents for the bars
pub struct Display {
    /// The percentage to show
    percent: u8,
    /// The ANSI color code
    ansi_code: String,
    /// Time in milliseconds to keep running
    time: u64,
}

const FILLED: &str = "█";
const EMPTY:  &str = " ";

pub fn display(args: Display) {
    let (num_characters, width) = calculate_width(args.percent);
    print!("\x1b[?25h\x1b[{}{}% [", args.ansi_code, args.percent);
    for i in 0..width {
        if i <= num_characters {
            print!("{}", FILLED);
        } else {
            print!("{}", EMPTY);
        }
    }
    print!("]\x1b[0m");
    // flush
    io::stdout().flush().expect("failed to flush wtf");
    let sleep_time = time::Duration::from_millis(args.time);
    thread::sleep(sleep_time);
}
