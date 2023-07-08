use std::{
    rc::Rc,
    thread::sleep,
    time::Duration
};

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
        ui::FieldEditable,
        popup::{centered_rect, draw_input},
        state::states::{PopupMode, InputMode}
    },
};

pub fn draw_popup<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
    where
        B: Backend,
{
    let system_index = app.planet_systems_list.state.selected().unwrap_or_default();

    let edit_path: String = match app.popup_state {
        PopupMode::PlanetSystem => app.planet_system_edit_list.edit_element.as_ref().unwrap().name.clone(),
        PopupMode::CenterStar => format!("{} -> {}",
                                         app.planet_system_edit_list.edit_element.as_ref().unwrap().name.clone(),
                                         app.center_star_edit_list.edit_element.as_ref().unwrap().name.clone()
        ),
        PopupMode::Planet => format!("{} -> {}",
                                     app.planet_system_edit_list.edit_element.as_ref().unwrap().name.clone(),
                                     app.planet_edit_list.edit_element.as_ref().unwrap().name.clone()
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

    let (edit_elements, state) = match app.popup_state {
        PopupMode::PlanetSystem => {
            let ell = get_elements(app.planet_system_edit_list.edit_element.as_ref());
            app.planet_system_edit_list.size = ell.len();
            (ell, &mut app.planet_system_edit_list.state)
        },
        PopupMode::CenterStar => {
            let ell = get_elements(app.center_star_edit_list.edit_element.as_ref());
            app.center_star_edit_list.size = ell.len();
            (ell, &mut app.center_star_edit_list.state)
        },
        PopupMode::Planet => {
            let ell = get_elements(app.planet_edit_list.edit_element.as_ref());
            app.planet_edit_list.size = ell.len();
            (ell, &mut app.planet_edit_list.state)
        },
        _ => (vec![], &mut app.planet_system_edit_list.state)
    };

    f.render_stateful_widget(get_tasks(edit_elements), chunks[0], state);

    draw_input(f, app, chunks[1]);
}

fn get_elements<T: FieldEditable>(element: Option<&T>) -> Vec<ListItem<'static>> {
    element.unwrap().get_field().iter()
        .map(|f| ListItem::new(
            Line::from(format!("{}: {}", f.name, f.value))
        ))
        .collect()
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

