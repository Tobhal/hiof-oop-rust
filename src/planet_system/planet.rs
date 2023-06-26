use std::error::Error;
use std::thread::sleep;
use std::time::Duration;
use crate::planet_system::moon::Moon;
use crate::util::ui::{FieldEditable, NoFieldError};

#[derive(Debug, Clone)]
pub struct Planet {
    pub name: String,
    pub mass: f32,
    pub radius: f32,
    pub semi_major_axis: f32,
    pub eccentricity: f32,
    pub orbital_period: f32,
    pub moons: Vec<Moon>
}

impl From<Vec<String>> for Planet {
    fn from(value: Vec<String>) -> Self {
        Planet {
            name: value[0].to_string(),
            mass: value[1].parse::<f32>().unwrap_or_default(),
            radius: value[2].parse::<f32>().unwrap_or_default(),
            semi_major_axis: value[3].parse::<f32>().unwrap_or_default(),
            eccentricity: value[4].parse::<f32>().unwrap_or_default(),
            orbital_period: value[5].parse::<f32>().unwrap_or_default(),
            moons: vec![],
        }
    }
}

impl FieldEditable for Planet {
    fn edit_field(&mut self, index: usize, value: String) -> Result<(), Box<dyn Error>> {
        // println!("{:#?}", index);
        // sleep(Duration::from_secs(5));
        match index {
            0 => self.name = value,
            1 => self.mass = match value.parse() {
                Ok(val) => val,
                Err(e) => {
                    // sleep(Duration::from_secs(5));
                    return Err(Box::new(e));
                }
            },
            2 => self.radius = value.parse()?,
            3 => self.semi_major_axis = value.parse()?,
            4 => self.eccentricity = value.parse()?,
            5 => self.orbital_period = value.parse()?,
            i => {
                return Err(Box::new(NoFieldError(index)));
            }
        }

        Ok(())
    }
}