use std::{
    fmt::format,
    rc::Rc,
    thread::sleep,
    time::Duration
};
use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs, Wrap, Clear},
    Frame,
};
use crate::{
    app::{
        views::{
            popup::draw_popup,
            tab1::draw_first_tab,
            find::draw_find_popup
        },
        app::App
    },
    util::{
        state::states::{InputMode, PopupMode}
    }
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(0)
            ]
                .as_ref()
        )
        .split(f.size());

    draw_status_line(f, app, chunks[0], vec![
        "'Q' = quit",
        "enter = select/edit",
        "'esc' = cancel",
        "'c' = close popup",
        "'f' = find"
    ]);
    draw_tabs(f, app, chunks[1]);

    match app.tabs.index {
        0 => draw_first_tab(f, app, chunks[2]),
        _ => {}
    };

    match app.popup_state {
        PopupMode::PlanetSystem | PopupMode::CenterStar | PopupMode::Planet => draw_popup(f, app, f.size()),
        PopupMode::Find => draw_find_popup(f, app, f.size()),
        _ => {}
    }
}

/*
Draw status line
 */
fn generate_elements(elements: Vec<String>) -> Vec<Span<'static>> {
    let mut spans: Vec<Span> = Vec::new();
    for (i, element) in elements.iter().enumerate() {
        spans.push(Span::from(element.clone()));
        // do not append " | " after last element
        if i < elements.len() - 1 {
            spans.push(Span::from(" | "));
        }
    }
    spans
}

fn draw_status_line<B>(f: &mut Frame<B>, app: &mut App, area: Rect, elements: Vec<&'static str>)
where
    B: Backend,
{
    let spans = generate_elements(elements.into_iter().map(|e| e.to_string()).collect());
    f.render_widget(
        Paragraph::new(
            vec![
                Line::from(spans)
            ]
        )
        .wrap(Wrap {
            trim: true
        }),
        area
    );
}

/*
Draw tabs
 */
fn draw_tabs<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
    where
        B: Backend,
{
    f.render_widget(
        Tabs::new(
            app
                .tabs
                .titles
                .iter()
                .map(|t| Line::from(Span::styled(*t, Style::default())))
                .collect()
        )
        .block(Block::default()
            .borders(Borders::ALL)
            .title(app.title))
        .highlight_style(Style::default()
            .add_modifier(Modifier::BOLD))
        .select(app.tabs.index),
        area
    );
}
