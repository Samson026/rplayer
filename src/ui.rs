use ratatui::{
    Frame,
    widgets::{Block, Paragraph},
};

use crate::app::{App, Screen};

pub fn draw(frame: &mut Frame, app: &App) {
    match app.screen {
        Screen::Home => draw_home(frame, app),
    }
}

pub fn draw_home(frame: &mut Frame, app: &App) {
    let text = if let Some(ref playing) = app.playing {
        format!("Currently playing: {}", playing)
    } else {
        "No file playing".to_string()
    };

    let para = Paragraph::new(text).block(Block::bordered());

    frame.render_widget(para, frame.area());
}
