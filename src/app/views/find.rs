use ratatui::backend::Backend;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Rect, Layout};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem};
use crate::app::app::{App, PopupMode};
use crate::app::views::popup::draw_popup;
use crate::app::views::popup_util::{centered_rect, draw_input};

pub fn draw_find_popup<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
    where
        B: Backend,
{
    let popup_area = centered_rect(50, 60, f.size());
    let block = Block::default()
        .title("Find planet system")
        .borders(Borders::ALL);

    f.render_widget(Clear, popup_area);
    f.render_widget(block, popup_area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
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

    let list_element: Vec<ListItem> = planet_system_names
        .iter()
        .map(|ps| {
            ListItem::new(
                vec![
                    Line::from(
                        Span::raw(
                            ps
                        )
                    )
                ]
            )
        })
        .collect();

    let list = List::new(list_element)
        .highlight_style(Style::default()
            .add_modifier(Modifier::BOLD)
        )
        .highlight_symbol("> ");

    f.render_stateful_widget(list, area, &mut app.find_list.state);
}
