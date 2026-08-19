use std::fs::File;

use clap::{Parser, Subcommand};
use rodio::Decoder;

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
    Play { file_path: String },
}

fn main() {
    let args = Args::parse();

    match args.cmd {
        Commands::Play { file_path } => play_mp3(&file_path),
    }
}

fn play_mp3(file_path: &str) {
    // Get an OS-Sink handle to the default physical sound device.
    // Note that the playback stops when the handle is dropped.//!
    let handle = rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
    let player = rodio::Player::connect_new(&handle.mixer());
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = File::open(file_path).unwrap();
    // Decode that sound file into a source
    let source = Decoder::try_from(file).unwrap();
    // Play the sound directly on the device
    handle.mixer().add(source);

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    std::thread::sleep(std::time::Duration::from_secs(5));
}
