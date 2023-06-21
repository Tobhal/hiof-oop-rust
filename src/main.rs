mod planet_system;
mod app;
mod util;

use std::error::Error;
use std::time::Duration;
use crate::planet_system::planet_system::PlanetSystem;
use crate::app::termion::run;

fn main() -> Result<(), Box<dyn Error>> {
    let planet_systems = PlanetSystem::new_systems_from_file("planets.csv".to_string());

    println!("{planet_systems:#?}");

    // run(Duration::from_millis(250), true, &planet_systems)?;

    Ok(())
}
