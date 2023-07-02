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
        ui::FieldEditable,
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

    draw_status_line(f, app, chunks[0]);
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
fn draw_status_line<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
    where
        B: Backend,
{
    f.render_widget(
        Paragraph::new(
            vec![
                Line::from(vec![
                    Span::from("'q' = quit"),
                    Span::from(" | "),
                    Span::from("'enter' = select/edit"),
                    Span::from(" | "),
                    Span::from("'esc' = cancel"),
                    Span::from(" | "),
                    Span::from("'c' = close popup"),
                    Span::from(" | "),
                    Span::from("'f' = find"),
                ]),
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
