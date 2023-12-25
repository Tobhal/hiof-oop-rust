use crate::{
    planet_system::{
        center_star::CenterStar,
        planet::Planet,
    }
};

pub struct PlanetSystemsCSV {
    pub name: String,
    pub center_star: CenterStar,
    pub planet: Planet
}


impl TryFrom<String> for PlanetSystemsCSV {
    type Error = &'static str;

    fn try_from(line: String) -> Result<Self, Self::Error> {
        let line_split: Vec<String> = line.split(',')
            .map(|x| x.to_string())
            .collect();

        if line_split.len() == 14 {
            Ok(
                PlanetSystemsCSV {
                    name: line_split[0].to_string(),
                    center_star: CenterStar {
                        name: line_split[2].to_string(),
                        mass: line_split[3].parse::<f32>().unwrap(),
                        radius: line_split[4].parse::<f32>().unwrap(),
                        effective_temperature: line_split[5].parse::<f32>().unwrap(),
                    },
                    planet: Planet {
                        name: line_split[7].to_string(),
                        mass: line_split[8].parse::<f32>().unwrap(),
                        radius: line_split[9].parse::<f32>().unwrap(),
                        semi_major_axis: line_split[10].parse::<f32>().unwrap(),
                        eccentricity: line_split[11].parse::<f32>().unwrap(),
                        orbital_period: line_split[12].parse::<f32>().unwrap(),
                        moons: vec![],
                    },
                }
            )
        } else {
            println!("{line_split:?}");
            Err("Wrong size for input string. Size should be 9 after splitting on ','.")
        }
    }
}