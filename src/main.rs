mod app;
mod ui;

use std::{fs::File, io::BufReader};

use clap::{Parser, Subcommand};
use ratatui::Terminal;
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
    Start { file_path: String },
}

fn main() {
    let args = Args::parse();

    match args.cmd {
        Commands::Play { file_path } => play_mp3(&file_path),
        Commands::Start { file_path } => run_tui(&file_path),
    }
}

fn play_mp3(file_path: &str) {
    // _stream must live as long as the sink
    let handle = rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
    let player = rodio::Player::connect_new(&handle.mixer());

    let file = BufReader::new(File::open(file_path).unwrap());
    let source = Decoder::try_from(file).unwrap();

    player.append(source);

    player.sleep_until_end();
}

fn run_tui(file_path: &str) {
    let mut terminal = ratatui::init();

    let mut app = App::new();
    app.playing = Some(file_path.clone().to_string());

    app.play(file_path);

    while app.running {
        terminal.draw(|frame| ui::draw(frame, &app));
        app.handle_event();
    }

    ratatui::restore();
}
