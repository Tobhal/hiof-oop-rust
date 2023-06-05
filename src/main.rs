use crate::planet_system::PlanetSystem;

mod planet;
mod center_star;
mod moon;
mod planet_system;
mod file_reader;
mod planet_system_csv;

fn main() {
    let planet_systems = PlanetSystem::new_system_from_file("solarSystem.csv".to_string(), "Solar system".to_string());

    println!("{planet_systems:#?}")
}
