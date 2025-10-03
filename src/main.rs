mod calculate_width;
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
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Display(args) => commands::display::display(args),
    }
}
