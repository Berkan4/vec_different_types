mod vec_enum;
mod vec_trait;
mod structs;
use vec_enum::enum_vec;
use vec_trait::trait_vec;

fn main() {
    println!("Hello, world!");
    enum_vec();
    trait_vec();
}


