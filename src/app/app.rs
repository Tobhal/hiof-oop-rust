use ratatui::{
    widgets::{ListItem, ListState},
    layout::Direction
};
use std::{
    thread,
    time,
    time::Duration,
    error::Error,
    thread::sleep,
};
use termion::event::Key;

use crate::{
    planet_system::{
        planet_system::PlanetSystem,
        center_star::CenterStar,
        planet::Planet,
    },
    util::{
        ui::FieldEditable,
        state::{
            list::StatefulList,
            tabs::TabsState,
            states::{PopupMode, InputMode}
        }
    }
};

pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub tabs: TabsState<'a>,
    pub enhanced_graphics: bool,

    pub planet_systems_list: StatefulList<'a, PlanetSystem>,
    pub planet_systems: Vec<PlanetSystem>,

    pub input_mode: InputMode,
    pub input: String,
    pub messages: Vec<String>,

    pub popup_state: PopupMode,

    pub planet_system_edit_list: StatefulList<'a, PlanetSystem>,
    pub planet_edit_list: StatefulList<'a, Planet>,
    pub center_star_edit_list: StatefulList<'a, CenterStar>,

    pub find_list: StatefulList<'a, PlanetSystem>
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, enhanced_graphics: bool, planet_systems: Vec<PlanetSystem>, planet_system_names: &'a Vec<String>) -> App<'a> {
        App {
            title,
            should_quit: false,
            tabs: TabsState::new(vec!["Planet Systesm"]),
            enhanced_graphics,

            planet_systems_list: StatefulList::new_with_items(planet_system_names),
            planet_systems,

            input_mode: InputMode::Normal,
            input: String::new(),
            messages: Vec::new(),

            popup_state: PopupMode::Hide,

            planet_system_edit_list: StatefulList::new_with_items(planet_system_names),
            planet_edit_list: StatefulList::new_with_items(planet_system_names),
            center_star_edit_list: StatefulList::new_with_items(planet_system_names),

            find_list: StatefulList::new_with_items(planet_system_names)
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
            PopupMode::CenterStar => {
                self.center_star_edit_list.previous_size();
            }
            PopupMode::Find => {
                self.find_list.previous_size();
            }
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
            PopupMode::CenterStar => {
                self.center_star_edit_list.next_size();
            }
            PopupMode::Find => {
                self.find_list.next_size();
            }
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
                    'f' => {
                        self.find_list.state.select(Some(0));
                        self.input = String::new();

                        self.popup_state = PopupMode::Find;
                        self.input_mode = InputMode::Editing;
                    },
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
                    'c' => {
                        self.planet_system_edit_list.state.select(Some(0));
                        self.popup_state = PopupMode::Hide;
                    },
                    '\n' => {
                        let planet_system_index = self.planet_systems_list.state.selected().unwrap_or_default();
                        let edit_index = self.planet_system_edit_list.state.selected().unwrap_or_default();

                        match edit_index {
                            0 => self.input_mode = InputMode::Editing,
                            1 => {
                                self.popup_state = PopupMode::CenterStar;
                                self.center_star_edit_list.edit_element = Some(self.planet_systems[planet_system_index].center_star.clone())
                            }
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
                    '\n' => {
                        self.input_mode = InputMode::Editing;
                    }
                    _ => {}
                }
            }
            (InputMode::Normal, PopupMode::CenterStar) => {
                match c {
                    'q' => self.should_quit = true,
                    'c' => {
                        self.planet_systems_list.state.select(Some(0));
                        self.center_star_edit_list.state.select(Some(0));
                        self.popup_state = PopupMode::Hide;
                    },
                    '\n' => {
                        self.input_mode = InputMode::Editing;
                    }
                    _ => {}
                }
            }
            (InputMode::Normal, PopupMode::Find) => {
                match c {
                    'q' => self.should_quit = true,
                    '\n' => {
                        todo!("Select planet system");
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
                            _ => {
                                planet_system.planets[edit_index-2].name = message.clone();
                                planet_system_edit.planets[edit_index-2].name = message.clone();
                            }
                        }

                        self.input_mode = InputMode::Normal;
                    },
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
                    },
                    c => self.input.push(c),
                }
            }
            (InputMode::Editing, PopupMode::CenterStar) => {
                match c {
                    '\n' => {
                        let message: String = self.input.drain(..).collect();

                        let planet_system_index = self.planet_systems_list.state.selected().unwrap_or_default();
                        let center_star_edit_index = self.center_star_edit_list.state.selected().unwrap_or_default();

                        match self.planet_systems[planet_system_index].center_star.edit_field(center_star_edit_index, message.clone()) {
                            Ok(_) => {}
                            Err(e) => {
                                println!("{:#?}", e.to_string());
                                sleep(Duration::from_secs(5));
                                self.input_mode = InputMode::Normal;

                                // Return before next function is run to not print two error messages to the screen.
                                return Ok(())
                            }
                        };
                        self.center_star_edit_list.edit_element.as_mut().unwrap().edit_field(center_star_edit_index, message.clone())?;

                        self.input_mode = InputMode::Normal;
                    },
                    c => self.input.push(c)
                }
            }
            (InputMode::Editing, PopupMode::Find) => {
                match c {
                    '\n' => {
                        let planet_system: Vec<(usize, String)> = self.planet_systems.iter().enumerate()
                            .filter(|(i, ps)| ps.name.contains(self.input.as_str()) || self.input.is_empty())
                            .map(|(i, ps)| (i, ps.name.clone()))
                            .collect();

                        let index = planet_system[self.find_list.state.selected().unwrap_or_default()].0;

                        self.planet_system_edit_list.edit_element = Some(self.planet_systems[index].clone());
                        self.planet_systems_list.state.select(Some(index));

                        self.input_mode = InputMode::Normal;
                        self.popup_state = PopupMode::PlanetSystem;
                    }
                    c => {
                        self.input.push(c);
                    }
                }
            }
            _ => {}
        }

        Ok(())
    }

    pub fn on_backspace(&mut self) -> Result<(), Box<dyn Error>> {
        match self.input_mode {
            InputMode::Normal => {}
            InputMode::Editing => {
                self.input.pop();
            }
        }

        Ok(())
    }

    pub fn on_esc(&mut self) -> Result<(), Box<dyn Error>> {
        match self.input_mode {
            InputMode::Normal => {
                match self.popup_state {
                    PopupMode::Hide => {}
                    PopupMode::PlanetSystem => self.popup_state = PopupMode::Hide,
                    PopupMode::CenterStar | PopupMode::Planet => self.popup_state = PopupMode::PlanetSystem,
                    PopupMode::Find => self.popup_state = PopupMode::Hide
                }
            }
            InputMode::Editing => {
                self.input_mode = InputMode::Normal
            }
        }

        Ok(())
    }

    pub fn on_tick(&mut self) -> Result<(), Box<dyn Error>> {
        // Update progress

        Ok(())
    }
}