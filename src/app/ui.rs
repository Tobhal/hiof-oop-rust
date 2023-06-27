use std::fmt::format;
use std::rc::Rc;
use std::thread::sleep;
use std::time::Duration;
use crate::app::app::{App, InputMode, PopupMode};
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs, Wrap, },
    Frame,
};
use ratatui::layout::Alignment;
use ratatui::text::Text;
use ratatui::widgets::Clear;
use crate::util::ui::FieldEditable;

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

}

/*
Draw status line
 */
fn draw_status_line<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
    where
        B: Backend,
{
    let text = vec![
        Line::from(vec![
            Span::from("'q' = quit"),
            Span::from(" | "),
            Span::from("'enter' = select/edit"),
            Span::from(" | "),
            Span::from("'esc' = cancel"),
            Span::from(" | "),
            Span::from("'c' = close popup"),
        ]),
    ];

    let paragraph = Paragraph::new(text)
        .wrap(Wrap {
            trim: true
        });

    f.render_widget(paragraph, area);
}

/*
Draw tabs
 */
fn draw_tabs<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
    where
        B: Backend,
{
    let titles = app
        .tabs
        .titles
        .iter()
        .map(|t| Line::from(Span::styled(*t, Style::default())))
        .collect();

    let tabs = Tabs::new(titles)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(app.title))
        .highlight_style(Style::default()
            .add_modifier(Modifier::BOLD))
        .select(app.tabs.index);

    // draw_test(f, app, chunks[0]);

    f.render_widget(tabs, area);
}

fn draw_first_tab<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
    where
        B: Backend,
{
    let index = app.planet_systems_list.state.selected().unwrap_or_default();

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(60),
                Constraint::Percentage(40),
            ]
                .as_ref(),
        )
        .split(area);

    let block = Block::default()
        .title("Content")
        .borders(Borders::ALL)
        .style(Style::default());

    f.render_widget(block, chunks[1]);

    draw_list(f, app, chunks[0]);
    draw_planet_system_info(f, app, chunks[1], index);

    if app.popup_state != PopupMode::Hide {
        draw_popup(f, app, f.size())
    }
}

/*
Draw lists
 */
fn draw_list<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
    where
        B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(90)])
        .direction(Direction::Horizontal)
        .split(area);

    let mut planet_system_names: Vec<String> = vec![];

    app.planet_systems.iter()
        .for_each(|s| planet_system_names.push(s.name.clone()));

    // Draw tasks
    let tasks: Vec<ListItem> = planet_system_names
        .iter()
        .map(|p| ListItem::new(vec![Line::from(Span::raw(p))]))
        .collect();

    let tasks = List::new(tasks)
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Systems")
        )
        .highlight_style(Style::default()
            .add_modifier(Modifier::BOLD)
        )
        .highlight_symbol("> ");

    f.render_stateful_widget(tasks, chunks[0], &mut app.planet_systems_list.state);
}

/*
Draw popup
 */
fn draw_popup<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
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
fn draw_edit_planet_system_list<B>(f: &mut Frame<B>, app: &mut App, chunks: &Rc<[Rect]>)
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

fn draw_edit_planet_list<B>(f: &mut Frame<B>, app: &mut App, chunks: &Rc<[Rect]>)
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

fn draw_edit_center_start_list<B>(f: &mut Frame<B>, app: &mut App, chunks: &Rc<[Rect]>)
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

/*
Draw info
 */
fn draw_planet_system_info<B>(f: &mut Frame<B>, app: &mut App, area: Rect, index: usize)
    where
        B: Backend,
{
    let planet_system = app.planet_systems[index].clone();

    let mut text = vec![
        // Line::from(index.to_string()),
        Line::from(planet_system.clone().name),
        Line::from(vec![
            Span::from("- Center star: "),
            Span::from(planet_system.center_star.name.clone().to_string())
        ]),
        Line::from(vec![
            Span::from(format!("- Num planets ({}): ", planet_system.planets.len().to_string())),
        ]),
    ];

    planet_system.planets.iter()
        .for_each(|p| text.push(Line::from(format!("- - {}", p.name.clone()))));

    let block = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(
        "System info",
        Style::default()
    ));

    let paragraph = Paragraph::new(text)
        .block(block)
        .wrap(Wrap {
            trim: true
        });

    f.render_widget(paragraph, area);
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
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