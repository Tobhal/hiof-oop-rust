use std::rc::Rc;

use ratatui::{
    backend::Backend,
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph}
};

use crate::{
    app::app::{App, InputMode, PopupMode},
    util::ui::FieldEditable
};

pub fn draw_popup<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
    where
        B: Backend
{
    let system_index = app.planet_systems_list.state.selected().unwrap_or_default();

    let edit_path: String = match app.popup_state {
        PopupMode::Hide => String::new(),
        PopupMode::PlanetSystem => app.planet_system_edit_list.edit_element.as_ref().unwrap().name.clone(),
        PopupMode::CenterStar => format!("{} -> {}",
                                         app.planet_system_edit_list.edit_element.as_ref().unwrap().name.clone(),
                                         app.center_star_edit_list.edit_element.as_ref().unwrap().name.clone()
        ),
        PopupMode::Planet => format!("{} -> {}",
                                     app.planet_system_edit_list.edit_element.as_ref().unwrap().name.clone(),
                                     app.planet_edit_list.edit_element.as_ref().unwrap().name.clone()
        )
    };

    let block = Block::default()
        .title(format!("Edit: {}", edit_path))
        .borders(Borders::ALL);

    let popup_area = centered_rect(60, 60, f.size());

    f.render_widget(Clear, popup_area); //this clears out the background
    f.render_widget(block, popup_area);

    if app.popup_state != PopupMode::Hide {
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

        match app.popup_state {
            PopupMode::Hide => {}
            PopupMode::PlanetSystem => draw_edit_planet_system_list(f, app, &chunks),
            PopupMode::CenterStar => draw_edit_center_start_list(f, app, &chunks),
            PopupMode::Planet => draw_edit_planet_list(f, app, &chunks),
        }

        let input = Paragraph::new(app.input.as_str())
            .style(match app.input_mode {
                InputMode::Normal => Style::default(),
                InputMode::Editing => Style::default().fg(Color::Yellow),
            })
            .block(Block::default().borders(Borders::ALL).title("Input"));

        f.render_widget(input, chunks[1]);
    }
}

/*
Draw edit list
 */
pub fn draw_edit_planet_system_list<B>(f: &mut Frame<B>, app: &mut App, chunks: &Rc<[Rect]>)
    where
        B: Backend,
{
    let mut planet_system_names: Vec<String> = vec![];

    app.planet_systems.iter()
        .for_each(|s| planet_system_names.push(s.name.clone()));

    // Draw tasks
    let mut edit_elements: Vec<ListItem> = planet_system_names
        .iter()
        .map(|p| ListItem::new(vec![Line::from(Span::raw(p))]))
        .collect();

    edit_elements = vec![
        ListItem::new(
            vec![
                Line::from(
                    format!("Name: {}", app.planet_system_edit_list.edit_element.as_ref().unwrap().name.to_string())
                )
            ]
        ),
        ListItem::new(
            vec![
                Line::from(
                    format!("Center star name: {}", app.planet_system_edit_list.edit_element.as_ref().unwrap().center_star.name.to_string())
                )
            ]
        ),
    ];

    app.planet_system_edit_list.edit_element.as_ref().unwrap().planets
        .iter()
        .for_each(|p| edit_elements.push(ListItem::new(
            vec![
                Line::from(
                    format!("Planet: {}", p.name)
                )
            ]
        )));

    app.planet_system_edit_list.size = edit_elements.len();

    let tasks = List::new(edit_elements)
        .block(Block::default())
        .highlight_style(Style::default()
            .add_modifier(Modifier::BOLD)
        )
        .highlight_symbol("> ");

    f.render_stateful_widget(tasks, chunks[0], &mut app.planet_system_edit_list.state);
}

pub fn draw_edit_planet_list<B>(f: &mut Frame<B>, app: &mut App, chunks: &Rc<[Rect]>)
    where
        B: Backend,
{
    // Draw tasks
    let edit_elements: Vec<ListItem> = app.planet_edit_list.edit_element
        .as_ref()
        .unwrap()
        .get_field()
        .iter()
        .map(|field| ListItem::new(
            Line::from(format!("{}: {}", field.name, field.value))
        )).collect();

    app.planet_edit_list.size = edit_elements.len();

    let tasks = List::new(edit_elements)
        .block(Block::default())
        .highlight_style(Style::default()
            .add_modifier(Modifier::BOLD)
        )
        .highlight_symbol("> ");

    f.render_stateful_widget(tasks, chunks[0], &mut app.planet_edit_list.state);
}

pub fn draw_edit_center_start_list<B>(f: &mut Frame<B>, app: &mut App, chunks: &Rc<[Rect]>)
    where
        B: Backend,
{
    // Draw tasks
    let edit_elements: Vec<ListItem> = app.center_star_edit_list.edit_element
        .as_ref()
        .unwrap()
        .get_field()
        .iter()
        .map(|field| ListItem::new(
            Line::from(format!("{}: {}", field.name, field.value))
        )).collect();

    app.center_star_edit_list.size = edit_elements.len();

    let tasks = List::new(edit_elements)
        .block(Block::default())
        .highlight_style(Style::default()
            .add_modifier(Modifier::BOLD)
        )
        .highlight_symbol("> ");

    f.render_stateful_widget(tasks, chunks[0], &mut app.center_star_edit_list.state);
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
                .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
                .as_ref(),
        )
        .split(popup_layout[1])[1]
}
