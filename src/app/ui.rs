use std::fmt::format;
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
            Span::from("'\'' = exit edit"),
            Span::from(" | "),
            Span::from("'.' = backspace"),
            Span::from(" | "),
            Span::from("'p' = show/hide popup"),
        ]),
    ];

    let paragraph = Paragraph::new(text)
        .wrap(Wrap {
            trim: true
        });

    f.render_widget(paragraph, area);
}

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
    let index = app.systems_list.state.selected().unwrap_or_default();

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(75),
                Constraint::Percentage(25),
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

    if app.show_popup {

        let block = Block::default()
            .title(format!("Edit {}", app.planet_systems[index].name.clone()))
            .borders(Borders::ALL);

        let popup_area = centered_rect(60, 60, f.size());

        f.render_widget(Clear, popup_area); //this clears out the background
        f.render_widget(block, popup_area);

        draw_edit_list(f, app, popup_area)
    }
}

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

    f.render_stateful_widget(tasks, chunks[0], &mut app.systems_list.state);
}

fn draw_edit_list<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
    where
        B: Backend,
{
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
        .split(area);

    let mut planet_system_names: Vec<String> = vec![];

    app.planet_systems.iter()
        .for_each(|s| planet_system_names.push(s.name.clone()));

    // Draw tasks
    let mut tasks: Vec<ListItem> = planet_system_names
        .iter()
        .map(|p| ListItem::new(vec![Line::from(Span::raw(p))]))
        .collect();
    
    tasks = vec![
        ListItem::new(
            vec![
                Line::from(
                    format!("Name: {}", app.system_edit.as_ref().unwrap().name.to_string())
                )
            ]
        ),
        ListItem::new(
            vec![
                Line::from(
                    format!("Center star name: {}", app.system_edit.as_ref().unwrap().center_star.name.to_string())
                )
            ]
        ),
    ];

    app.system_edit.as_ref().unwrap().planets
        .iter()
        .for_each(|p| tasks.push(ListItem::new(
            vec![
                Line::from(
                    format!("Planet: {}", p.name)
                )
            ]
        )));

    app.system_edit_list.size = tasks.len();

    let tasks = List::new(tasks)
        .block(Block::default())
        .highlight_style(Style::default()
            .add_modifier(Modifier::BOLD)
        )
        .highlight_symbol("> ");

    f.render_stateful_widget(tasks, chunks[0], &mut app.system_edit_list.state);

    let input = Paragraph::new(app.input.as_str())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title("Input"));

    f.render_widget(input, chunks[1]);
}

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