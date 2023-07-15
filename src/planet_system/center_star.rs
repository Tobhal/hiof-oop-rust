use std::error::Error;
use crate::util::ui::FieldEditable;

#[derive(Debug, Clone, Default, FieldEditable)]
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
