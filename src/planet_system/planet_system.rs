use std::{
    collections::HashMap,
    mem,
};
use std::error::Error;

use crate::{
    planet_system::{
        center_star::CenterStar,
        planet::Planet,
        moon::Moon,
        planet_system_csv::PlanetSystemsCSV
    },
    util::file_reader::read_lines,
};
use crate::util::ui::{Field, FieldEditable, NoFieldError};

pub enum Types {
    CenterStar(CenterStar),
    Planet(Planet, String),
    Moon(Moon, String),
}

impl From<Vec<String>> for Types {
    fn from(value: Vec<String>) -> Self {
        let c = value[6].clone().to_string();

        match value[7].as_str() {
            "sun" => Types::CenterStar(CenterStar::from(value)),
            "planet" => Types::Planet(Planet::from(value), c),
            "moon" => {
                Types::Moon(Moon::from(value), c)
            },
            v => panic!("Unknown type: {}", v)
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct PlanetSystem {
    pub name: String,
    pub center_star: CenterStar,
    pub planets: Vec<Planet>
}

impl PlanetSystem {
    pub fn new() -> PlanetSystem {
        PlanetSystem {
            name: "".to_string(),
            center_star: CenterStar::new(),
            planets: vec![],
        }
    }

    pub fn new_systems_from_file(file_name: String) -> Vec<PlanetSystem> {
        let mut planets: HashMap<String, Vec<Planet>> = HashMap::new();
        let mut stars: HashMap<String, CenterStar> = HashMap::new();

        read_lines(file_name).enumerate()
            .filter(|(i, _)| i.clone() != 0)
            .filter(|(_, line)| !line.as_ref().unwrap().is_empty())
            .for_each(|(_, line)| {
                let line = line.unwrap();
                let planet_system_line = PlanetSystemsCSV::try_from(line).unwrap();

                stars.entry(planet_system_line.name.clone())
                    .or_insert(planet_system_line.center_star);

                let mut planet = planet_system_line.planet.to_owned();
                let new_planet = planet_system_line.planet.to_owned();

                planets.entry(planet_system_line.name.clone())
                    .and_modify(|list| list.push(mem::replace(&mut planet, new_planet)))
                    .or_insert(vec![planet]);
            });

        let mut planet_systems: Vec<PlanetSystem> = vec![];

        planets.iter()
            .for_each(|(system_name, planets)| {
                let mut planet_list: Vec<Planet> = vec![];

                planets.iter().for_each(|p| planet_list.push(p.clone()));

                planet_systems.push(PlanetSystem {
                    name: system_name.clone(),
                    center_star: stars.get(system_name).unwrap().clone(),
                    planets: planet_list,
                })

            });

        planet_systems
    }
}

impl FieldEditable for PlanetSystem {
    fn edit_field(&mut self, index: usize, value: String) -> Result<(), Box<dyn Error>> {
        match index {
            0 => self.name = value,
            i => {
                return Err(Box::new(NoFieldError(i)));
            }
        }

        Ok(())
    }

    fn get_field(&self) -> Vec<Field> {
        let mut fields = vec![
            Field { name: "Name", value: self.name.to_string() },
            Field { name: "Center star", value: self.center_star.name.to_string() },
        ];

        self.planets.iter()
            .for_each(|p| fields.push(
                Field { name: "Planet", value: p.name.to_string() }
            ));

        fields
    }
}