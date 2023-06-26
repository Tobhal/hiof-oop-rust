use ratatui::widgets::ListState;
use crate::planet_system::planet_system::PlanetSystem;

use std::{thread, time};
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;
use ratatui::layout::Direction;
use crate::planet_system::planet::Planet;
use crate::util::ui::FieldEditable;

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

pub struct StatefulList<'l, T, E> {
    pub state: ListState,
    pub items: &'l Vec<T>,
    pub size: usize,
    pub edit_element: Option<E>
}

impl<'l, T, E> StatefulList<'l, T, E> {
    pub fn new_with_items(items: &Vec<T>) -> StatefulList<T, E> {
        StatefulList {
            state: ListState::default(),
            items,
            size: 0,
            edit_element: None
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

    pub planet_systems_list: StatefulList<'a, String, PlanetSystem>,
    pub planet_systems: Vec<PlanetSystem>,

    pub input_mode: InputMode,
    pub input: String,
    pub messages: Vec<String>,

    pub popup_state: PopupMode,

    pub planet_system_edit_list: StatefulList<'a, String, PlanetSystem>,

    pub planet_edit_list: StatefulList<'a, String, Planet>,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, enhanced_graphics: bool, planet_systems: Vec<PlanetSystem>, planet_system_names: &'a Vec<String>) -> App<'a> {
        App {
            title,
            should_quit: false,
            tabs: TabsState::new(vec!["Planets"]),
            enhanced_graphics,

            planet_systems_list: StatefulList::new_with_items(planet_system_names),
            planet_systems,

            input_mode: InputMode::Normal,
            input: String::new(),
            messages: Vec::new(),

            popup_state: PopupMode::Hide,

            planet_system_edit_list: StatefulList::new_with_items(planet_system_names),

            planet_edit_list: StatefulList::new_with_items(planet_system_names),
        }
    }

    pub fn on_up(&mut self) -> Result<(), Box<dyn Error>> {
        match self.popup_state {
            PopupMode::Hide => {
                self.planet_systems_list.previous();
            }
            PopupMode::PlanetSystem => {
                self.planet_system_edit_list.previous_size();
            }
            PopupMode::Planet => {
                self.planet_edit_list.previous_size();
            }
            PopupMode::CenterStar => {}
        }

        Ok(())
    }

    pub fn on_down(&mut self) -> Result<(), Box<dyn Error>> {
        match self.popup_state {
            PopupMode::Hide => {
                self.planet_systems_list.next();
            }
            PopupMode::PlanetSystem => {
                self.planet_system_edit_list.next_size();
            }
            PopupMode::Planet => {
                self.planet_edit_list.next_size();
            }
            PopupMode::CenterStar => {}
        }

        Ok(())
    }

    pub fn on_right(&mut self) -> Result<(), Box<dyn Error>> {
        self.tabs.next();

        Ok(())
    }

    pub fn on_left(&mut self) -> Result<(), Box<dyn Error>> {
        self.tabs.previous();

        Ok(())
    }

    pub fn on_key(&mut self, c: char) -> Result<(), Box<dyn Error>> {
        match (self.input_mode.clone(), self.popup_state.clone()) {
            (InputMode::Normal, PopupMode::Hide) => {
                match c {
                    'q' => self.should_quit = true,
                    'c' => self.popup_state = PopupMode::Hide,
                    '\n' => {
                        let index = self.planet_systems_list.state.selected().unwrap_or_default();

                        self.popup_state = PopupMode::PlanetSystem;
                        self.planet_system_edit_list.edit_element = Some(self.planet_systems[index].clone());
                    }
                    _ => {}
                }
            }
            (InputMode::Normal, PopupMode::PlanetSystem) => {
                match c {
                    'q' => self.should_quit = true,
                    'c' | 'p' => {
                        self.planet_system_edit_list.state.select(Some(0));
                        self.popup_state = PopupMode::Hide;
                    },
                    '\n' => {
                        let planet_system_index = self.planet_systems_list.state.selected().unwrap_or_default();
                        let edit_index = self.planet_system_edit_list.state.selected().unwrap_or_default();

                        match edit_index {
                            0 | 1 => self.input_mode = InputMode::Editing,
                            _ => {
                                self.popup_state = PopupMode::Planet;
                                self.planet_edit_list.edit_element = Some(self.planet_systems[planet_system_index].planets[edit_index-2].clone())
                            }
                        }
                    }
                    _ => {}
                }
            }
            (InputMode::Normal, PopupMode::Planet) => {
                match c {
                    'q' => self.should_quit = true,
                    'c' => {
                        self.planet_systems_list.state.select(Some(0));
                        self.planet_edit_list.state.select(Some(0));
                        self.popup_state = PopupMode::Hide;
                    },
                    'p' => {
                        self.planet_edit_list.state.select(Some(0));
                        self.popup_state = PopupMode::PlanetSystem;
                    },
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

                        let system_index = self.planet_systems_list.state.selected().unwrap_or_default();
                        let edit_index = self.planet_system_edit_list.state.selected().unwrap_or_default();

                        let planet_system = &mut self.planet_systems[system_index];
                        let planet_system_edit = self.planet_system_edit_list.edit_element.as_mut().unwrap();

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

                        let planet_system_index = self.planet_systems_list.state.selected().unwrap_or_default();
                        let planet_system_edit_index = self.planet_system_edit_list.state.selected().unwrap_or_default() - 2;
                        let planet_edit_index = self.planet_edit_list.state.selected().unwrap_or_default();

                        match self.planet_systems[planet_system_index].planets[planet_system_edit_index].edit_field(planet_edit_index, message.clone()) {
                            Ok(_) => {}
                            Err(e) => {
                                println!("{:#?}", e.to_string());
                                sleep(Duration::from_secs(5));
                                self.input_mode = InputMode::Normal;

                                // Return before next function is run to not print two error messages to the screen.
                                return Ok(())
                            }
                        };
                        self.planet_edit_list.edit_element.as_mut().unwrap().edit_field(planet_edit_index,message.clone())?;

                        self.input_mode = InputMode::Normal;
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

        Ok(())
    }

    pub fn on_tick(&mut self) -> Result<(), Box<dyn Error>> {
        // Update progress

        Ok(())
    }
}