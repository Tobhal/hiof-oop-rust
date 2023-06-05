use std::collections::HashMap;
use std::mem;
use crate::center_star::CenterStar;
use crate::planet::Planet;
use crate::file_reader::read_lines;
use crate::moon::Moon;
use crate::planet_system_csv::PlanetSystemCSV;

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

#[derive(Debug)]
pub struct PlanetSystem {
    pub name: String,
    pub center_star: CenterStar,
    pub planets: Vec<Planet>
}

impl PlanetSystem {
    pub fn new_system_from_file(file_name: String, name: String) -> PlanetSystem {
        let mut center_star: CenterStar = CenterStar::new();
        let mut planets: Vec<Planet> = vec![];
        let mut moons: HashMap<String, Vec<Moon>> = HashMap::new();

        read_lines(file_name).enumerate()
            .filter(|(i, _)| i.clone() != 0)
            .filter(|(_, line)| !line.as_ref().unwrap().is_empty())
            .for_each(|(_, line)| {
                let line = line.unwrap();

                match PlanetSystemCSV::try_from(line).unwrap().t {
                    Types::CenterStar(s) => center_star = s,
                    Types::Planet(p, _) => planets.push(p),
                    Types::Moon(m, c) => {
                        let mut moon = m.to_owned();
                        let new_moon = m.to_owned();

                        moons.entry(c)
                            .and_modify(|list| list.push(mem::replace(&mut moon, new_moon)))
                            .or_insert(vec![moon]);
                    }
                }
            });

        planets.iter_mut()
            .filter(|p| moons.contains_key(&*p.name))
            .for_each(|p| {
                moons.get(&*p.name).unwrap().iter()
                    .for_each(|m| {
                        p.moons.push(m.clone());
                    })
            });

        PlanetSystem {
            name,
            center_star,
            planets,
        }
    }
}

