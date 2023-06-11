use crate::app::app::App;
use ratatui::{
    backend::Backend,
    layout::{
        Constraint,
        Direction,
        Layout,
        Rect
    },
    style::{
        Color,
        Modifier,
        Style
    },
    text::{
        Line,
        Span
    },
    widgets::{
        Block, Borders, List, ListItem, Paragraph, Tabs, Wrap,
    },
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

    let mut planet_names: Vec<String> = vec![];

    app.planet_system.planets.iter()
        .for_each(|p| planet_names.push(p.name.clone()));

    // Draw tasks
    let tasks: Vec<ListItem> = planet_names
        .iter()
        .map(|p| ListItem::new(vec![Line::from(Span::raw(p))]))
        .collect();

    let tasks = List::new(tasks)
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Planets")
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
    let planet = app.planet_system.planets[index as usize].clone();
    let mut moons: String = String::new();

    planet.moons.iter().for_each(|m| {
        moons.push_str(&*m.name);
        moons.push_str(", ");
    });

    let text = vec![
        // Line::from(index.to_string()),
        Line::from(planet.clone().name),
        Line::from(vec![
            Span::from("- Mass: "),
            Span::from(format!("{:e}", planet.mass))
        ]),
        Line::from(vec![
            Span::from("- Radius: "),
            Span::from(planet.clone().radius.to_string())
        ]),
        Line::from(vec![
            Span::from("- Semi major axis: "),
            Span::from(planet.clone().semi_major_axis.to_string())
        ]),
        Line::from(vec![
            Span::from("- Eccentricity: "),
            Span::from(planet.clone().eccentricity.to_string())
        ]),
        Line::from(vec![
            Span::from("- Orbital period: "),
            Span::from(planet.clone().orbital_period.to_string())
        ]),
        Line::from(vec![
            Span::from("- moons "),
            Span::from(planet.moons.len().to_string()),
            Span::from(": "),
            Span::from(moons)
        ]),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(
        "Planet info",
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    ));

    let paragraph = Paragraph::new(text)
        .block(block)
        .wrap(Wrap {
            trim: true
        });

    f.render_widget(paragraph, area);
}
