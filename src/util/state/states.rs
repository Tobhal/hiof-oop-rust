#[derive(Debug, PartialEq, Clone)]
pub enum InputMode {
    Normal,
    Editing
}

#[derive(Debug, PartialEq, Clone)]
pub enum PopupMode {
    Hide,
    PlanetSystem,
    CenterStar,
    Planet,
    Find
}

