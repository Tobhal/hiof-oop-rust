use crate::app::app::App;
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs, Wrap, },
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());

    let titles = app
        .tabs
        .titles
        .iter()
        .map(|t| Line::from(Span::styled(*t, Style::default().fg(Color::Green))))
        .collect();

    let tabs = Tabs::new(titles)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(app.title))
        .highlight_style(Style::default()
            .fg(Color::Yellow))
        .select(app.tabs.index);

    f.render_widget(tabs, chunks[0]);

    match app.tabs.index {
        0 => draw_first_tab(f, app, chunks[1]),
        _ => {}
    };
}

fn draw_first_tab<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
    where
        B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ]
                .as_ref(),
        )
        .split(area);

    draw_list(f, app, chunks[0]);

    draw_text(f, chunks[1], app.tasks.state.selected().unwrap_or_default() as i16, app);
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

    f.render_stateful_widget(tasks, chunks[0], &mut app.tasks.state);
}

fn draw_text<B>(f: &mut Frame<B>, area: Rect, index: i16, app: &mut App)
    where
        B: Backend,
{
    // let planet = app.planet_systems.planets[index as usize].clone();
    // let mut moons: String = String::new();

    let planet_system = app.planet_systems[index as usize].clone();

    let text = vec![
        // Line::from(index.to_string()),
        Line::from(planet_system.clone().name),
        Line::from(vec![
            Span::from("- Center star: "),
            Span::from(planet_system.center_star.name.clone().to_string())
        ]),
        Line::from(vec![
            Span::from("- Num planets: "),
            Span::from(planet_system.planets.len().to_string())
        ])
    ];

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
