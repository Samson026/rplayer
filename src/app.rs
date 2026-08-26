use std::{
    fs::File,
    io::{BufReader, Error},
    path::PathBuf,
    time::Duration,
};

use crossterm::event::{self, Event, KeyCode};
use rodio::{Decoder, MixerDeviceSink, Player, Source};

pub struct App {
    pub playing: Option<Playing>,
    pub running: bool,
    pub screen: Screen,
    pub cursor: usize,
    #[allow(dead_code)]
    handle: MixerDeviceSink,
    pub player: Player,
    pub songs: Vec<PathBuf>,
    pub search_text: String,
}

pub struct Playing {
    pub song: String,
    pub duration: Option<Duration>,
}

pub enum Screen {
    Home,
}

impl App {
    pub fn new() -> Self {
        let handle = rodio::DeviceSinkBuilder::open_default_sink().expect("");
        let player = rodio::Player::connect_new(handle.mixer());

        Self {
            running: true,
            playing: None,
            screen: Screen::Home,
            handle,
            player,
            songs: Vec::new(),
            cursor: 0,
            search_text: String::new(),
        }
    }

    pub fn play(&self, file_path: &str) -> Option<Playing> {
        let file = BufReader::new(File::open(file_path).unwrap());
        let source = Decoder::try_from(file).unwrap();
        let duration = source.total_duration();
        self.player.stop();
        self.player.append(source);
        Some(Playing {
            song: file_path.to_string(),
            duration,
        })
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
                KeyCode::Char(' ') => {
                    if self.player.is_paused() {
                        self.player.play();
                    } else {
                        self.player.pause();
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }
}
