use ratatui::widgets::ListState;
use crate::planet_system::planet_system::PlanetSystem;

use std::{thread, time};
use std::thread::sleep;
use std::time::Duration;
use crate::planet_system::planet::Planet;

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
    pub size: usize,
}

impl<'l, T> StatefulList<'l, T> {
    pub fn new_with_items(items: &Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
            size: 0
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => (i + 1) % self.items.len(),
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn next_size(&mut self) {
        let i = match self.state.selected() {
            Some(i) => (i + 1) % self.size,
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

    pub fn previous_size(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.size - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

#[derive(Debug, Clone)]
pub enum InputMode {
    Normal,
    Editing
}

#[derive(Debug, PartialEq, Clone)]
pub enum PopupMode {
    Hide,
    PlanetSystem,
    CenterStar,
    Planet,
}

pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub tabs: TabsState<'a>,
    pub enhanced_graphics: bool,

    pub systems_list: StatefulList<'a, String>,
    pub planet_systems: Vec<PlanetSystem>,

    pub input_mode: InputMode,
    pub input: String,
    pub messages: Vec<String>,

    pub popup_state: PopupMode,

    pub system_edit_list: StatefulList<'a, String>,
    pub system_edit: Option<PlanetSystem>,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, enhanced_graphics: bool, planet_systems: Vec<PlanetSystem>, planet_system_names: &'a Vec<String>) -> App<'a> {
        App {
            title,
            should_quit: false,
            tabs: TabsState::new(vec!["Planets"]),
            enhanced_graphics,

            systems_list: StatefulList::new_with_items(planet_system_names),
            planet_systems,

            input_mode: InputMode::Normal,
            input: String::new(),
            messages: Vec::new(),

            popup_state: PopupMode::Hide,

            system_edit_list: StatefulList::new_with_items(planet_system_names),
            system_edit: None,
        }
    }

    pub fn on_up(&mut self) {
        match self.popup_state {
            PopupMode::Hide => {
                self.systems_list.previous();
            }
            PopupMode::PlanetSystem | PopupMode::Planet | PopupMode::CenterStar => {
                self.system_edit_list.previous_size();
            }
        }
    }

    pub fn on_down(&mut self) {
        match self.popup_state {
            PopupMode::Hide => {
                self.systems_list.next();
            }
            PopupMode::PlanetSystem | PopupMode::Planet | PopupMode::CenterStar => {
                self.system_edit_list.next_size();
            }
        }
    }

    pub fn on_right(&mut self) {
        self.tabs.next();
    }

    pub fn on_left(&mut self) {
        self.tabs.previous();
    }

    pub fn on_key(&mut self, c: char) {
        match (self.input_mode.clone(), self.popup_state.clone()) {
            (InputMode::Normal, PopupMode::Hide) => {
                match c {
                    'q' => self.should_quit = true,
                    'c' => self.popup_state = PopupMode::Hide,
                    '\n' => {
                        let index = self.systems_list.state.selected().unwrap_or_default();

                        self.popup_state = PopupMode::PlanetSystem;
                        self.system_edit = Some(self.planet_systems[index].clone());
                    }
                    _ => {}
                }
            }
            (InputMode::Normal, PopupMode::PlanetSystem) => {
                match c {
                    'q' => self.should_quit = true,
                    'c' | 'p' => self.popup_state = PopupMode::Hide,
                    '\n' => {
                        let system_index = self.systems_list.state.selected().unwrap_or_default();
                        let edit_index = self.system_edit_list.state.selected().unwrap_or_default();

                        match edit_index {
                            0 | 1 => self.input_mode = InputMode::Editing,
                            _ => {

                                self.popup_state = PopupMode::Planet;
                                self.system_edit_list.state.select(Some(0));
                            }
                        }
                    }
                    _ => {}
                }
            }
            (InputMode::Normal, PopupMode::Planet) => {
                match c {
                    'q' => self.should_quit = true,
                    'c' | 'p' => self.popup_state = PopupMode::Hide,
                    '\n' => {
                        self.input_mode = InputMode::Editing;
                    }
                    _ => {}
                }
            }

            (InputMode::Editing, PopupMode::PlanetSystem) => {
                match c {
                    '\n' => {
                        // Push edited line to the current editing line
                        let message: String = self.input.drain(..).collect();
                        // self.messages.push(self.input.drain(..).collect());

                        let system_index = self.systems_list.state.selected().unwrap_or_default();
                        let edit_index = self.system_edit_list.state.selected().unwrap_or_default();

                        let planet_system = &mut self.planet_systems[system_index];
                        let planet_system_edit = self.system_edit.as_mut().unwrap();

                        match edit_index {
                            0 => {
                                planet_system.name = message.clone();
                                planet_system_edit.name = message.clone();
                            },
                            1 => {
                                planet_system.center_star.name = message.clone();
                                planet_system_edit.center_star.name = message.clone();
                            },
                            2..=100 => {
                                planet_system.planets[edit_index-2].name = message.clone();
                                planet_system_edit.planets[edit_index-2].name = message.clone();
                            }
                            _ => {}
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
            (InputMode::Editing, PopupMode::Planet) => {
                match c {
                    '\n' => {
                        let message: String = self.input.drain(..).collect();

                        let system_index = self.systems_list.state.selected().unwrap_or_default();
                        let edit_index = self.system_edit_list.state.selected().unwrap_or_default();

                        let planet = &mut self.planet_systems[system_index].planets[edit_index-2];
                        let planet_edit = &mut self.system_edit.as_mut().unwrap().planets[edit_index-2];

                        match edit_index {
                            1 => {
                                planet.name = message.clone();
                                planet_edit.name = message.clone();
                            }
                            _ => {}
                        }
                    }
                    '\r' | '\u{0008}' | '.' => {
                        self.input.pop();
                    }
                    '\u{001B}' | '\'' => self.input_mode = InputMode::Normal,
                    c => self.input.push(c)
                }
            }
            _ => {}
        }
    }

    pub fn on_tick(&mut self) {
        // Update progress
    }
}