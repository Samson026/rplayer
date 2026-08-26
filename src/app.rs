use std::{
    fs::File,
    io::{BufReader, Error},
    path::PathBuf,
};

use crossterm::event::{self, Event, KeyCode, KeyEvent};
use rodio::{Decoder, MixerDeviceSink, Player};

pub struct App {
    pub playing: Option<String>,
    pub running: bool,
    pub screen: Screen,
    pub cursor: usize,
    handle: MixerDeviceSink,
    player: Player,
    pub songs: Vec<PathBuf>,
}

pub enum Screen {
    Home,
}

impl App {
    pub fn new() -> Self {
        let handle = rodio::DeviceSinkBuilder::open_default_sink().expect("");
        let player = rodio::Player::connect_new(&handle.mixer());

        Self {
            running: true,
            playing: None,
            screen: Screen::Home,
            handle,
            player,
            songs: Vec::new(),
            cursor: 0,
        }
    }

    pub fn play(&self, file_path: &str) -> Option<String> {
        let file = BufReader::new(File::open(file_path).unwrap());
        let source = Decoder::try_from(file).unwrap();
        self.player.stop();
        self.player.append(source); 
        Some(file_path.to_string())
    }

    pub fn handle_event(&mut self) -> Result<(), Error> {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc => {
                    self.running = false;
                }
                KeyCode::Up => {
                    if self.cursor > 0 {
                        self.cursor -= 1;
                    }
                }
                KeyCode::Down => {
                    if self.cursor < self.songs.len() - 1 {
                        self.cursor += 1;
                    }
                }
                KeyCode::Enter => {
                    if let Some(song) = self.songs.get(self.cursor).unwrap().to_str() {
                        self.playing = self.play(song);
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }
}
