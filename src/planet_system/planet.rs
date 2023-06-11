use crate::planet_system::moon::Moon;

#[derive(Debug, Clone)]
pub struct Planet {
    pub name: String,
    pub mass: f32,
    pub radius: f32,
    pub semi_major_axis: f32,
    pub eccentricity: f32,
    pub orbital_period: u32,
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
            orbital_period: value[5].parse::<u32>().unwrap_or_default(),
            moons: vec![],
        }
    }
}