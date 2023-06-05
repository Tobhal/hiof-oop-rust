use crate::planet_system::Types;

#[allow(dead_code)]
pub struct PlanetSystemCSV {
    pub(crate) name: String,
    pub(crate) mass: f32,
    pub(crate) radius: f32,
    pub(crate) semi_major_axis: f32,
    pub(crate) eccentricity: f32,
    pub(crate) orbital_period: u32,
    pub(crate) central_celestial_body: String,
    pub(crate) effective_temperature: u16,
    pub(crate) t: Types
}

impl TryFrom<String> for PlanetSystemCSV {
    type Error = &'static str;

    fn try_from(line: String) -> Result<Self, Self::Error> {
        let line_split: Vec<String> = line.split(',')
            .map(|x| x.to_string())
            .collect();

        let name: String = line_split[0].to_string();

        if line_split.len() == 9 {
            Ok(
                PlanetSystemCSV {
                    name,
                    mass: line_split[1].parse::<f32>().unwrap_or_default(),
                    radius: line_split[2].parse::<f32>().unwrap_or_default(),
                    semi_major_axis: line_split[3].parse::<f32>().unwrap_or_default(),
                    eccentricity: line_split[4].parse::<f32>().unwrap_or_default(),
                    orbital_period: line_split[5].parse::<u32>().unwrap_or_default(),
                    central_celestial_body: line_split[6].to_string(),
                    //t: Types::from(line_split[7].to_string()),
                    effective_temperature: line_split[8].parse::<u16>().unwrap_or_default(),
                    t: Types::from(line_split),
                }
            )
        } else {
            println!("{line_split:?}");
            Err("Wrong size for input string. Size should be 9 after splitting on ','.")
        }

    }
}
