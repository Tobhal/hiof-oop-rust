use crate::app::{
    ui,
    app::App
};
use ratatui::{
    backend::{Backend, TermionBackend},
    Terminal,
};
use std::{error::Error, io, sync::mpsc, thread, time::Duration};
use std::sync::Arc;
use termion::{
    event::Key,
    input::{MouseTerminal, TermRead},
    raw::IntoRawMode,
    screen::IntoAlternateScreen,
};
use crate::planet_system::planet_system::PlanetSystem;

pub fn run(tick_rate: Duration, enhanced_graphics: bool, planet_system: Vec<PlanetSystem>) -> Result<(), Box<dyn Error>> {
    // setup terminal
    let stdout = io::stdout()
        .into_raw_mode()
        .unwrap()
        .into_alternate_screen()
        .unwrap();
    let stdout = MouseTerminal::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;


    let planet_system_names: Vec<String> = planet_system.iter().map(|p| p.name.clone()).collect();

    // create app and run it
    let app = App::new("Planet system", enhanced_graphics, planet_system, &planet_system_names);

    run_app(&mut terminal, app, tick_rate)?;

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> Result<(), Box<dyn Error>> {
    let events = events(tick_rate);
    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        match events.recv()? {
            Event::Input(key) => match key {
                Key::Char(c) => app.on_key(c),
                Key::Up => app.on_up(),
                Key::Down => app.on_down(),
                Key::Left => app.on_left(),
                Key::Right => app.on_right(),
                Key::Backspace => app.on_backspace(),
                Key::Esc => app.on_esc(),
                _ => {Ok(())}
            },
            Event::Tick => app.on_tick(),
        }.expect("Event error");

        if app.should_quit {
            return Ok(());
        }
    }
}

enum Event {
    Input(Key),
    Tick,
}

fn events(tick_rate: Duration) -> mpsc::Receiver<Event> {
    let (tx, rx) = mpsc::channel();
    let keys_tx = tx.clone();
    thread::spawn(move || {
        let stdin = io::stdin();
        for key in stdin.keys().flatten() {
            if let Err(err) = keys_tx.send(Event::Input(key)) {
                eprintln!("{err}");
                return;
            }
        }
    });
    thread::spawn(move || loop {
        if let Err(err) = tx.send(Event::Tick) {
            eprintln!("{err}");
            break;
        }
        thread::sleep(tick_rate);
    });
    rx
}
