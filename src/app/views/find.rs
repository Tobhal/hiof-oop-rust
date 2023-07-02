use ratatui::{
    backend::Backend,
    Frame,
    layout::{Constraint, Direction, Rect, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem},
};

use crate::{
    app::{
        app::App,
        views::popup::draw_popup,
    },
    util::{
        popup::{centered_rect, draw_input},
        state::states::{InputMode, PopupMode}
    }
};

pub fn draw_find_popup<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
    where
        B: Backend,
{
    let popup_area = centered_rect(50, 60, f.size());

    f.render_widget(Clear, popup_area);
    f.render_widget(
        Block::default()
        .title("Find planet system")
        .borders(Borders::ALL),
        popup_area
    );

    let chunks = Layout::default()
        .margin(1)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(1),
            ]
                .as_ref(),
        )
        .split(popup_area);

    draw_input(f, app, chunks[0]);
    draw_find_planet_system_list(f, app, chunks[1]);
}

pub fn draw_find_planet_system_list<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
    where
        B: Backend,
{
    let planet_system_names: Vec<String> = app.planet_systems.iter()
        .filter(|ps| ps.name.contains(app.input.as_str()) || app.input.is_empty())
        .map(|ps| ps.name.clone())
        .collect();

    app.find_list.size = planet_system_names.len();

    f.render_stateful_widget(
        List::new(planet_system_names
            .iter()
            .map(|ps| {
                ListItem::new(
                    vec![
                        Line::from(
                            Span::raw(ps)
                        )
                    ]
                )
            })
            .collect::<Vec<ListItem>>()
        )
            .highlight_style(Style::default()
                .add_modifier(Modifier::BOLD)
            )
            .highlight_symbol("> "),
        area,
        &mut app.find_list.state
    );
}
