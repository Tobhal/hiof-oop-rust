use std::sync::Arc;
use ratatui::widgets::ListState;
use crate::planet_system::planet::Planet;
use crate::planet_system::planet_system::PlanetSystem;

pub struct TabsState<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> TabsState<'a> {
    pub fn new(titles: Vec<&'a str>) -> TabsState {
        TabsState { titles, index: 0 }
    }
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}

pub struct StatefulList<'l, T> {
    pub state: ListState,
    pub items: &'l Vec<T>,
}

impl<'l, T> StatefulList<'l, T> {
    pub fn with_items(items: &Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => (i + 1) % self.items.len(),
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

impl<'l, PlanetSystem> StatefulList<'l, PlanetSystem> {
    pub fn with_planet_systems(items: &Vec<PlanetSystem>) -> StatefulList<PlanetSystem> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }
}

pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub tabs: TabsState<'a>,
    pub systems_list: StatefulList<'a, PlanetSystem>,
    pub planet_systems: &'a Vec<PlanetSystem>,
    pub enhanced_graphics: bool,
    pub show_popup: bool
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, enhanced_graphics: bool, planet_systems: &'a Vec<PlanetSystem>) -> App<'a> {
        App {
            title,
            should_quit: false,
            tabs: TabsState::new(vec!["Planets"]),
            systems_list: StatefulList::with_items(planet_systems),
            planet_systems,
            enhanced_graphics,
            show_popup: false
        }
    }

    pub fn on_up(&mut self) {
        self.systems_list.previous();
    }

    pub fn on_down(&mut self) {
        self.systems_list.next();
    }

    pub fn on_right(&mut self) {
        self.tabs.next();
    }

    pub fn on_left(&mut self) {
        self.tabs.previous();
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => { self.should_quit = true; }
            'p' => { self.show_popup = !self.show_popup }
            '\n' => { self.show_popup = !self.show_popup }
            _ => {}
        }
    }

    pub fn on_tick(&mut self) {
        // Update progress
    }
}