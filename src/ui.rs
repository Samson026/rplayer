use crossterm::cursor;
use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Style, Stylize},
    widgets::{Block, List, ListItem, ListState, Paragraph, Wrap},
};

use crate::app::{App, Screen};

pub fn draw(frame: &mut Frame, app: &App) {
    match app.screen {
        Screen::Home => draw_home(frame, app),
    }
}

pub fn draw_home(frame: &mut Frame, app: &App) {
    let [left_area, right_area] =
        Layout::horizontal([Constraint::Percentage(30), Constraint::Percentage(70)])
            .areas(frame.area());

    // playing

    let playing_block = Block::bordered().title("Now playing");
    let pb_area = playing_block.inner(left_area);
    frame.render_widget(playing_block, left_area);

    let [name_area, details_area] =
        Layout::vertical([Constraint::Length(3), Constraint::Length(3)]).areas(pb_area);

    let text = if let Some(ref playing) = app.playing {
        format!("Currently playing: {}", playing.song)
    } else {
        "No file playing".to_string()
    };

    let para1 = Paragraph::new(text.clone()).wrap({ Wrap { trim: true } });

    frame.render_widget(para1, name_area);

    let ratio = app.playing.as_ref().and_then(|p| {
        p.duration
            .map(|d| (app.player.get_pos().as_secs_f64() / d.as_secs_f64()).min(1.0))
    });

    if let Some(ratio) = ratio {
        let [_, gauge_area, _] = Layout::vertical([
            Constraint::Min(0),
            Constraint::Length(1),
            Constraint::Min(0),
        ])
        .areas(details_area);

        let width = gauge_area.width as usize;
        let filled = (width as f64 * ratio) as usize;
        let bar = "█".repeat(filled.min(width)) + &"░".repeat(width.saturating_sub(filled));

        let gauge = Paragraph::new(bar).style(Style::default().fg(Color::White));

        frame.render_widget(gauge, gauge_area);
    }

    // Other songs
    let list_block = Block::bordered().title("Playlist");
    let lb_area = list_block.inner(right_area);
    frame.render_widget(list_block, right_area);

    // let constraints: Vec<Constraint> = app.songs.iter().map(|_| Constraint::Length(3)).collect();
    // let areas = Layout::vertical(constraints).split(lb_area);

    let songs_list: Vec<ListItem> = app
        .songs
        .iter()
        .map(|song| {
            let name = song
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Unknown");

            ListItem::new(name)
        })
        .collect();

    let mut state = ListState::default();
    state.select(Some(app.cursor));

    let list = List::new(songs_list)
        .block(Block::bordered())
        .highlight_style(Style::default().bg(Color::White).fg(Color::Black))
        .highlight_symbol(">");

    frame.render_stateful_widget(list, right_area, &mut state);

    // for (i, (song, area)) in app.songs.iter().zip(areas.iter()).enumerate() {
    //     let song_name = song
    //         .file_stem()
    //         .and_then(|s| s.to_str())
    //         .unwrap_or("Unknown");

    //     let style = if i == app.cursor {
    //         Style::default().bg(Color::Gray).fg(Color::White)
    //     } else {
    //         Style::default()
    //     };

    //     let para = Paragraph::new(song_name)
    //         .style(style)
    //         .block(Block::bordered());

    //     frame.render_widget(para, *area);
    // }
}
