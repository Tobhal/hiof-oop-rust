#![allow(dead_code, unused_assignments, unused_imports, unused_variables)]

mod planet_system;
mod app;
mod util;

use std::error::Error;
use std::time::Duration;
use crate::planet_system::planet_system::PlanetSystem;
use crate::app::termion::run;

fn main() -> Result<(), Box<dyn Error>> {
    let mut planet_systems = PlanetSystem::new_systems_from_file("planets_100.csv".to_string());

    // println!("{planet_systems:#?}");

    run(Duration::from_millis(250), true, &mut planet_systems).unwrap();

    Ok(())
}
