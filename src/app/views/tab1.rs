use ratatui::{
    backend::Backend,
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
};

use crate::app::app::App;

pub fn draw_first_tab<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
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

    draw_list(f, app, chunks[0]);
    draw_planet_system_info(f, app, chunks[1], index);
}

pub fn draw_list<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
    where
        B: Backend,
{
    let planet_system_names: Vec<String> = app.planet_systems.iter()
        .map(|ps| ps.name.clone())
        .collect();

    let list_elements: Vec<ListItem> = planet_system_names
        .iter()
        .map(|p| ListItem::new(vec![Line::from(Span::raw(p))]))
        .collect();

    let list = List::new(list_elements)
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Systems")
        )
        .highlight_style(Style::default()
            .add_modifier(Modifier::BOLD)
        )
        .highlight_symbol("> ");

    f.render_stateful_widget(list, area, &mut app.planet_systems_list.state);
}

/*
Draw info
 */
pub fn draw_planet_system_info<B>(f: &mut Frame<B>, app: &mut App, area: Rect, index: usize)
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
