use std::error::Error;

use crate::util::ui::{FieldEditable, NoFieldError, Field};

#[derive(Debug, Clone, Default)]
pub struct CenterStar {
    pub name: String,
    pub mass: f32,
    pub radius: f32,
    pub effective_temperature: f32,
}

impl CenterStar {
    pub(crate) fn new() -> CenterStar {
        CenterStar {
            name: "".to_string(),
            mass: 0.0,
            radius: 0.0,
            effective_temperature: 0.0,
        }
    }
}

impl From<Vec<String>> for CenterStar {
    fn from(value: Vec<String>) -> Self {
        CenterStar {
            name: value[0].to_string(),
            mass: value[1].parse::<f32>().unwrap_or_default(),
            radius: value[2].parse::<f32>().unwrap_or_default(),
            effective_temperature: value[8].parse::<f32>().unwrap_or_default(),
        }
    }
}

impl FieldEditable for CenterStar {
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
            3 => self.effective_temperature = match value.parse() {
                Ok(val) => val,
                Err(e) => return Err(Box::new(e))
            },
            i => return Err(Box::new(NoFieldError(i)))
        }
        
        Ok(())
    }

    fn get_field(&self) -> Vec<Field> {
        vec![
            Field { name: "Name", value: self.name.to_string() },
            Field { name: "Mass", value: format!("{:e}", self.mass) },
            Field { name: "Radius", value: format!("{:e}", self.radius) },
            Field { name: "Effective_temperature", value: format!("{:e}", self.effective_temperature) }
        ]
    }
}