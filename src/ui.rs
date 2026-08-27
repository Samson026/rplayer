use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
};

use crate::app::{App, Screen};

pub fn draw(frame: &mut Frame, app: &App) {
    let [title_area, content_area] =
        Layout::vertical([Constraint::Percentage(5), Constraint::Percentage(95)])
            .areas(frame.area());

    let title = format!("rPlayer-{}", env!("CARGO_PKG_VERSION"));

    let block = Block::default()
        .borders(Borders::TOP)
        .title(title)
        .title_alignment(Alignment::Center);

    frame.render_widget(block, title_area);

    match app.screen {
        Screen::Home => draw_home(frame, content_area, app),
    }
}

pub fn draw_home(frame: &mut Frame, area: Rect, app: &App) {
    let [left_area, right_area] =
        Layout::horizontal([Constraint::Percentage(30), Constraint::Percentage(70)]).areas(area);

    // playing

    let playing_block = Block::bordered().title("Now playing");
    let pb_area = playing_block.inner(left_area);
    frame.render_widget(playing_block, left_area);

    let [name_area, _, time_area, details_area, pause_area] = Layout::vertical([
        Constraint::Length(2),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(3),
        Constraint::Length(3),
    ])
    .areas(pb_area);

    let text = if let Some(ref playing) = app.playing {
        format!("Currently playing: {}", playing.song)
    } else {
        "No file playing".to_string()
    };

    let para1 = Paragraph::new(text.clone()).wrap(Wrap { trim: true });

    frame.render_widget(para1, name_area);

    // time
    let time = app.playing.as_ref().and_then(|p| {
        p.duration.map(|d| {
            let a_quot = app.player.get_pos().as_secs() / 60;
            let a_rem = app.player.get_pos().as_secs() % 60;
            let b_quot = d.as_secs() / 60;
            let b_rem = d.as_secs() % 60;
            format!("{}:{} / {}:{}", a_quot, a_rem, b_quot, b_rem)
        })
    });

    frame.render_widget(time, time_area);

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

    // pause display

    if app.player.is_paused() {
        let pause = Paragraph::new("Paused");
        frame.render_widget(pause, pause_area);
    }

    // Other songs

    // let constraints: Vec<Constraint> = app.songs.iter().map(|_| Constraint::Length(3)).collect();
    // let areas = Layout::vertical(constraints).split(lb_area);

    let [search_area, playlist_area] =
        Layout::vertical([Constraint::Length(3), Constraint::Min(0)]).areas(right_area);

    let search =
        Paragraph::new(app.search_text.to_string()).block(Block::bordered().title("Search"));

    frame.render_widget(search, search_area);

    let songs_list: Vec<ListItem> = app
        .filtered_songs
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
        .block(Block::bordered().title("Playlist"))
        .highlight_style(Style::default().bg(Color::White).fg(Color::Black))
        .highlight_symbol("> ");

    frame.render_stateful_widget(list, playlist_area, &mut state);
}
