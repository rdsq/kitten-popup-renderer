mod calculate_width;
mod brightnessctl;
mod amixer;
mod spawn_the_thing;
mod commands;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "kitten-popup-renderer")]
#[command(about = "Displaying TUI progress bars with kitty's kitten")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Display(commands::display::Display),
    Brightness(commands::brightness::Brightness),
    Volume(commands::volume::Volume),
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Display(args) => commands::display::display(args),
        Commands::Brightness(args) => commands::brightness::brightness(args),
        Commands::Volume(args) => commands::volume::volume(args),
    }
}
