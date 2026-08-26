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
    pub filtered_songs: Vec<PathBuf>,
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
            filtered_songs: Vec::new(),
        }
    }

    pub fn play(&self, file_path: &PathBuf) -> Option<Playing> {
        let file = BufReader::new(File::open(file_path).unwrap());
        let source = Decoder::try_from(file).unwrap();
        let duration = source.total_duration();
        self.player.stop();
        self.player.append(source);
        Some(Playing {
            song: file_path
                .file_stem()
                .and_then(|stem| stem.to_str())
                .unwrap_or("Unknown")
                .to_string(),
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
                    if let Some(song) = self.filtered_songs.get(self.cursor) {
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
                KeyCode::Char(char) => {
                    self.search_text.push(char);
                    self.calculate_search();
                }
                KeyCode::Backspace => {
                    self.search_text.pop();
                    self.calculate_search();
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn calculate_search(&mut self) {
        if self.search_text.is_empty() {
            self.filtered_songs = self.songs.clone();
            return;
        }

        self.filtered_songs = Vec::new();

        for song in &self.songs {
            if song
                .file_stem()
                .and_then(|stem| stem.to_str())
                .is_some_and(|stem| {
                    stem.to_lowercase()
                        .contains(&self.search_text.to_lowercase())
                })
            {
                self.filtered_songs.push(song.clone());
            }
        }

        self.filtered_songs.sort_by_key(|song| {
            song.file_stem()
                .and_then(|stem| stem.to_str())
                .and_then(|stem| stem.to_lowercase().find(&self.search_text.to_lowercase()))
                .unwrap_or(usize::MAX)
        });
    }
}
