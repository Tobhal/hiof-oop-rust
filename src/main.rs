#![allow(dead_code, unused_assignments, unused_imports, unused_variables)]

mod planet_system;
mod app;
mod util;

use std::{
    error::Error,
    time::Duration
};

use crate::{
    planet_system::planet_system::PlanetSystem,
    app::termion::run
};

fn main() -> Result<(), Box<dyn Error>> {
    let planet_systems = PlanetSystem::new_systems_from_file("planets_100.csv".to_string());

    run(Duration::from_millis(250), true, planet_systems)?;

    Ok(())
}
