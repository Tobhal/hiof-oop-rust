use std::{
    error::Error,
    thread::sleep,
    time::Duration
};

use crate::{
    planet_system::moon::Moon,
    util::ui::{FieldEditable, NoFieldError, Field}
};

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
        match index {
            0 => self.name = value,
            1 => self.mass = match value.parse() {
                Ok(val) => val,
                Err(e) => return Err(Box::new(e))
            },
            2 => self.radius = match value.parse() {
                Ok(val) => val,
                Err(e) => return Err(Box::new(e))
            },
            3 => self.semi_major_axis = match value.parse() {
                Ok(val) => val,
                Err(e) => return Err(Box::new(e))
            },
            4 => self.eccentricity = match value.parse() {
                Ok(val) => val,
                Err(e) => return Err(Box::new(e))
            },
            5 => self.orbital_period = match value.parse() {
                Ok(val) => val,
                Err(e) => return Err(Box::new(e))
            },
            i => {
                return Err(Box::new(NoFieldError(i)));
            }
        }

        Ok(())
    }

    fn get_field(&self) -> Vec<Field> {
        vec![
            Field { name: "Name", value: self.name.to_string() },
            Field { name: "Mass", value: format!("{:e}", self.mass) },
            Field { name: "Radius", value: format!("{:e}", self.radius) },
            Field { name: "Semi major axis", value: format!("{:e}", self.semi_major_axis) },
            Field { name: "eccentricity", value: format!("{:e}", self.eccentricity) },
            Field { name: "orbital period", value: format!("{:e}", self.orbital_period) }
        ]
    }
}