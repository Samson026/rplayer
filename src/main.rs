use clap::{Parser, Subcommand};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Play { Song: String },
}

fn main() {
    let args = Args::parse();

    match args.cmd {
        Commands::Play { Song } => todo!(),
    }
}
