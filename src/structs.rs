
#[derive(Debug, Default)]
pub struct Support {
    pub coordinates: Coordinates,
    wx: bool,
    wy: bool,
    wz: bool,
    px: bool,
    py: bool,
    pz: bool,
}

#[derive(Debug, Default)]
pub struct Bend {
    pub coordinates: Coordinates,
    bend_readius: f32,
}

#[derive(Debug, Default, Clone)]
pub struct Coordinates {
    x: f32,
    y: f32,
    z: f32,
}