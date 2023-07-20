#![allow(dead_code, unused_assignments, unused_imports, unused_variables)]

mod planet_system;
mod app;
mod util;

use {
    std::{
        error::Error,
        time::Duration
    },
    crate::{
        planet_system::planet_system::PlanetSystem,
        app::termion::run,
        util::ui::FieldEditable,
    }
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut planet_systems = PlanetSystem::new_systems_from_file("planets_100.csv".to_string());
    planet_systems.sort_by_cached_key(|ps| ps.name.clone());

    run(Duration::from_millis(250), true, planet_systems)?;

    Ok(())
}
