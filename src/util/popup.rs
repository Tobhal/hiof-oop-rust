use ratatui::{
    backend::Backend,
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

use crate::{
    app::app::App,
    util::state::states::{PopupMode, InputMode}
};

/// helper function to create a centered rect using up certain percentage of the available rect `r`
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            rect_constraints(percent_x).as_ref(),
        )
        .split(
            Layout::default()
                .constraints(
                    rect_constraints(percent_y).as_ref(),
                )
                .split(r)[1]
        )[1]
}

fn rect_constraints(percent: u16) -> Vec<Constraint> {
    vec![
        Constraint::Percentage((100 - percent) / 2),
        Constraint::Percentage(percent),
        Constraint::Percentage((100 - percent) / 2),
    ]
}

pub fn draw_input<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
    where
        B: Backend
{
    f.render_widget(
        Paragraph::new(app.input.as_str())
            .style(match app.input_mode {
                InputMode::Normal => Style::default(),
                InputMode::Editing => Style::default().fg(Color::Yellow),
            })
            .block(Block::default().borders(Borders::ALL).title("Input")),
        area
    )
}
