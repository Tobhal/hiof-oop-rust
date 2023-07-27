use std::{
    error::Error,
    fmt
};

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

pub use field_editable_derive::FieldEditable;
pub struct NoFieldError(pub usize);

impl fmt::Display for NoFieldError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No field with index = {}", self.0)
    }
}

impl fmt::Debug for NoFieldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "No field with index = {}", self.0)
    }
}

impl Error for NoFieldError {}

pub struct Field<'f> {
    pub(crate) name: &'f str,
    pub(crate) value: String
}

pub trait FieldEditable {
    fn get_fields(&self) -> Vec<(&'static str, String)>;
    fn edit_field(&mut self, field: &'static str, value: String) -> Result<(), Box<dyn Error>>;
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