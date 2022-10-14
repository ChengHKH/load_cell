use crate::app::App;
use tui::{
    backend::Backend,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Tabs},
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());
    let titles = app
        .tabs
        .titles
        .iter()
        .map(|t| Spans::from(Span::styled(*t, Style::default().add_modifier(Modifier::ITALIC))))
        .collect();
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(app.title))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .select(app.tabs.current_index);
    f.render_widget(tabs, chunks[0]);
    match app.tabs.current_index {
        0 => draw_scale_tab(f, app, chunks[1]),
        1 => draw_logger_tab(f, app, chunks[1]),
        2 => draw_options_tab(f, app, chunks[1]),
        3 => draw_quit_tab(f, app, chunks[1]),
        _ => {}
    };
}

fn draw_scale_tab<B: Backend>(
    f: &mut Frame<B>,
    app: &mut App,
    area: Rect) {

}

fn draw_logger_tab<B: Backend>(
    f: &mut Frame<B>,
    app: &mut App,
    area: Rect) {

}

fn draw_options_tab<B: Backend>(
    f: &mut Frame<B>,
    app: &mut App,
    area: Rect) {

}

fn draw_quit_tab<B: Backend>(
    f: &mut Frame<B>,
    app: &mut App,
    area: Rect) {

}