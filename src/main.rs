mod app;
mod ui;

use std::{
    fs::{self, File},
    io::BufReader,
    time::Duration,
};

use clap::{Parser, Subcommand};
use crossterm::event;
use rodio::Decoder;

use crate::app::App;

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
    Start { dir_path: String },
}

fn main() {
    let args = Args::parse();

    match args.cmd {
        Commands::Play { file_path } => play_mp3(&file_path),
        Commands::Start { dir_path } => run_tui(&dir_path),
    }
}

fn play_mp3(file_path: &str) {
    // _stream must live as long as the sink
    let handle = rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
    let player = rodio::Player::connect_new(handle.mixer());

    let file = BufReader::new(File::open(file_path).unwrap());
    let source = Decoder::try_from(file).unwrap();

    player.append(source);

    player.sleep_until_end();
}

fn run_tui(dir_path: &str) {
    let mut terminal = ratatui::init();

    let mut app = App::new();

    let files = fs::read_dir(dir_path).unwrap();

    for file in files {
        let entry = file.unwrap();
        let file = entry.path();

        if file.is_file() {
            app.songs.push(file);
        }
    }

    app.filtered_songs = app.songs.clone();

    while app.running {
        let _ = terminal.draw(|frame| ui::draw(frame, &app));

        if event::poll(Duration::from_millis(16)).unwrap() {
            app.handle_event().unwrap();
        }
    }

    ratatui::restore();
}
