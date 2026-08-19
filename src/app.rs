use std::{
    fs::File,
    io::{BufReader, Error},
};

use crossterm::event::{self, Event, KeyCode, KeyEvent};
use rodio::{Decoder, MixerDeviceSink, Player};

pub struct App {
    pub playing: Option<String>,
    pub running: bool,
    pub screen: Screen,
    handle: MixerDeviceSink,
    player: Player,
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
        }
    }

    pub fn play(&self, file_path: &str) {
        let file = BufReader::new(File::open(file_path).unwrap());
        let source = Decoder::try_from(file).unwrap();
        self.player.append(source);
    }

    pub fn handle_event(&mut self) -> Result<(), Error> {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc => {
                    self.running = false;
                }
                _ => {}
            }
        }

        Ok(())
    }
}
