use ratatui::{
    backend::Backend,
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
};

use crate::app::app::App;

pub fn draw_load_tab<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
    where
        B: Backend,
{
    let block = Block::default()
        .title("Load")
        .borders(Borders::ALL)
        .style(Style::default());

    f.render_widget(block, area);
}
