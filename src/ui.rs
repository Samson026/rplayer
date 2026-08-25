use crossterm::cursor;
use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Style, Stylize},
    widgets::{Block, Paragraph},
};

use crate::app::{App, Screen};

pub fn draw(frame: &mut Frame, app: &App) {
    match app.screen {
        Screen::Home => draw_home(frame, app),
    }
}

pub fn draw_home(frame: &mut Frame, app: &App) {
    let [left_area, right_area] =
        Layout::horizontal([Constraint::Percentage(20), Constraint::Percentage(80)])
            .areas(frame.area());

    let text = if let Some(ref playing) = app.playing {
        format!("Currently playing: {}", playing)
    } else {
        "No file playing".to_string()
    };

    let para1 = Paragraph::new(text.clone()).block(Block::bordered());

    frame.render_widget(para1, left_area);

    // Other songs
    let block = Block::bordered().title("Playlist");
    let inner_area = block.inner(right_area);
    frame.render_widget(block, right_area);

    let constraints: Vec<Constraint> = app.songs.iter().map(|_| Constraint::Length(3)).collect();
    let areas = Layout::vertical(constraints).split(inner_area);

    for (i, (song, area)) in app.songs.iter().zip(areas.iter()).enumerate() {
        let song_name = song
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Unknown");

        let style = if i == app.cursor {
            Style::default().bg(Color::Gray).fg(Color::White)
        } else {
            Style::default()
        };

        let para = Paragraph::new(song_name)
            .style(style)
            .block(Block::bordered());

        frame.render_widget(para, *area);

    }
}
