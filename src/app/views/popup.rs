use std::{
    rc::Rc,
    thread::sleep,
    time::Duration
};
use std::fmt::format;

use ratatui::{
    backend::Backend,
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph}
};
use ratatui::layout::Constraint::Min;

use crate::{
    app::app::App,
    util::{
        state::states::{PopupMode, InputMode},
        ui::{FieldEditable, centered_rect, draw_input},
    },
};

pub fn draw_popup<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
    where
        B: Backend,
{
    let system_index = app.planet_systems_list.state.selected().unwrap_or_default();

    let edit_path: String = match app.popup_state {
        PopupMode::PlanetSystem => app.edit_list.edit_element.as_ref().unwrap().name.clone(),
        PopupMode::CenterStar => format!("{} -> {}",
                                         app.edit_list.edit_element.as_ref().unwrap().name.clone(),
                                         app.edit_list.edit_element.as_ref().unwrap().center_star.name.clone()
        ),
        PopupMode::Planet => format!("{} -> {}",
                                     app.edit_list.edit_element.as_ref().unwrap().name.clone(),
                                     app.edit_list.edit_element.as_ref().unwrap().planets[app.edit_list.size].name.clone()
        ),
        _ => String::new(),
    };

    let popup_area = centered_rect(60, 60, f.size());

    f.render_widget(Clear, popup_area); //this clears out the background
    f.render_widget(
        Block::default()
        .title(format!("Edit: {}", edit_path))
        .borders(Borders::ALL),
        popup_area
    );

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Min(1),
                Constraint::Length(3),
            ]
                .as_ref(),
        )
        .split(popup_area);

    let edit_elements: Vec<ListItem>= match app.popup_state {
        PopupMode::PlanetSystem => {
            let edit_element = app.edit_list.edit_element.as_ref().unwrap();

            let mut ell_string: Vec<String> = edit_element
                .get_fields()
                .iter()
                .map(|f| format!("{}: {}", f.0, f.1))
                .collect();

            ell_string.push(format!("Center star: {}", edit_element.center_star.get_fields()[0].1));

            app.edit_list.edit_element
                .as_ref()
                .unwrap()
                .planets
                .iter()
                .for_each(|p| ell_string.push(
                    format!("Planet: {}", p.get_fields()[0].1)
                ));

            app.edit_list.items = ell_string.clone();

            let ell: Vec<ListItem<'static>> = ell_string
                .iter()
                .map(|s| ListItem::new(
                    Line::from(s.to_string())
                ))
                .collect();

            ell
        },
        PopupMode::CenterStar => {
            let ell_string: Vec<String> = app.edit_list.edit_element
                .as_ref()
                .unwrap()
                .center_star
                .get_fields()
                .iter()
                .map(|f| format!("{}: {}", f.0, f.1))
                .collect();

            app.edit_list.items = ell_string.clone();

            let ell: Vec<ListItem<'static>> = ell_string
                .iter()
                .map(|s| ListItem::new(
                    Line::from(s.to_string())
                ))
                .collect();

            ell
        },
        PopupMode::Planet => {
            let ell_string: Vec<String> = app.edit_list.edit_element
                .as_ref()
                .unwrap()
                .planets[app.edit_list.size]
                .get_fields()
                .iter()
                .map(|f| format!("{}: {}", f.0, f.1))
                .collect();

            app.edit_list.items = ell_string.clone();

            let ell: Vec<ListItem<'static>> = ell_string
                .iter()
                .map(|s| ListItem::new(
                    Line::from(s.to_string())
                ))
                .collect();

            ell
        },
        _ => vec![]
    };

    f.render_stateful_widget(get_tasks(edit_elements), chunks[0], &mut app.edit_list.state);

    draw_input(f, app, chunks[1]);
}

fn get_tasks(tasks: Vec<ListItem>) -> List {
    List::new(tasks)
        .block(Block::default())
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
        )
        .highlight_symbol("> ")
}

