
#[derive(Debug, Default)]
pub struct Support {
    pub coordinates: Coordinates,
    // Boundary conditions: true = dirichlet, false = neumann
    // w is refering to translational movement
    wx: bool,
    wy: bool,
    wz: bool,
    // p is refering to rotational movement
    px: bool,
    py: bool,
    pz: bool,
}

#[derive(Debug, Default)]
pub struct Spring {
    pub coordinates: Coordinates,
    stiffness: f32,
}

#[derive(Debug, Default, Clone)]
pub struct Coordinates {
    x: f32,
    y: f32,
    z: f32,
}