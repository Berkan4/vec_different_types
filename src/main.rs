mod vec_enum;
mod vec_trait;
use vec_enum::enum_vec;
use vec_trait::trait_vec;

fn main() {
    println!("Hello, world!");
    enum_vec();
    trait_vec();
}

#[derive(Debug, Default)]
struct Support {
    wx: bool,
    wy: bool,
    wz: bool,
    px: bool,
    py: bool,
    pz: bool,
    coordinates: Coordinates
}

#[derive(Debug, Default)]
struct Bend {
    coordinates: Coordinates
}

#[derive(Debug, Default, Clone)]
struct Coordinates {
    x: f32,
    y: f32,
    z: f32,
}

