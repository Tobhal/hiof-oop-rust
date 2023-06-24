use ratatui::widgets::ListState;
use crate::planet_system::planet_system::PlanetSystem;

use std::{thread, time};
use std::thread::sleep;
use std::time::Duration;

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

    pub fn next_size(&mut self, size: usize) {
        let i = match self.state.selected() {
            Some(i) => (i + 1) % size,
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

    pub fn previous_size(&mut self, size: usize) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    size - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

}

#[derive(Debug)]
pub enum InputMode {
    Normal,
    Editing
}

#[derive(Debug, PartialEq)]
pub enum PopupMode {
    Hide,
    Show,
}

pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub tabs: TabsState<'a>,
    pub systems_list: StatefulList<'a, String>,
    pub planet_systems: Vec<PlanetSystem>,
    pub enhanced_graphics: bool,

    pub input_mode: InputMode,
    pub input: String,
    pub messages: Vec<String>,

    pub show_popup: bool,

    pub system_edit_list: StatefulList<'a, String>,
    pub system_edit: Option<PlanetSystem>,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, enhanced_graphics: bool, planet_systems: Vec<PlanetSystem>, planet_system_names: &'a Vec<String>) -> App<'a> {
        App {
            title,
            should_quit: false,
            tabs: TabsState::new(vec!["Planets"]),
            systems_list: StatefulList::with_items(planet_system_names),
            planet_systems,
            enhanced_graphics,
            input_mode: InputMode::Normal,
            input: String::new(),
            messages: Vec::new(),

            show_popup: false,

            system_edit_list: StatefulList::with_items(planet_system_names),
            system_edit: None,
        }
    }

    pub fn on_up(&mut self) {
        if self.show_popup {
            self.system_edit_list.previous_size(2);
        } else {
            self.systems_list.previous();
        }
    }

    pub fn on_down(&mut self) {
        if self.show_popup {
            self.system_edit_list.next_size(2);
        } else {
            self.systems_list.next();
        }
    }

    pub fn on_right(&mut self) {
        self.tabs.next();
    }

    pub fn on_left(&mut self) {
        self.tabs.previous();
    }

    pub fn on_key(&mut self, c: char) {
        match self.input_mode {
            InputMode::Normal => {
                match c {
                    'q' => { self.should_quit = true; }
                    'p' => { self.show_popup = !self.show_popup }
                    '\n' => {
                        let index = self.systems_list.state.selected().unwrap_or_default();

                        if self.show_popup {
                            self.input_mode = InputMode::Editing;
                        } else {
                            self.show_popup = !self.show_popup;
                            self.system_edit = Some(self.planet_systems[index].clone())
                        }

                    }
                    _ => {}
                }
            }
            InputMode::Editing => {
                match c {
                    '\n' => {
                        // Push edited line to the current editing line
                        let message = self.input.drain(..).collect();
                        // self.messages.push(self.input.drain(..).collect());

                        let system_index = self.systems_list.state.selected().unwrap_or_default();
                        let edit_index = self.system_edit_list.state.selected().unwrap_or_default();

                        if edit_index == 0 {
                            self.planet_systems[system_index].name = message;
                        } else if edit_index == 1 {
                            self.planet_systems[system_index].center_star.name = message;
                        }

                        self.input_mode = InputMode::Normal;
                    },
                    '\r' | '\u{0008}' | '.' => {
                        self.input.pop();
                    }
                    '\u{001B}' | '\'' => self.input_mode = InputMode::Normal,
                    c => self.input.push(c)
                }
            }
        }
    }

    pub fn on_tick(&mut self) {
        // Update progress
    }
}