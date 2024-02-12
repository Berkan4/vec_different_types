mod vec_enum;
mod vec_trait;
mod structs;
mod vec_alt_struct;


use vec_enum::enum_vec;
use vec_trait::trait_vec;
use crate::vec_alt_struct::alt_strust_vec;

fn main() {
    println!("Hello, world!");
    enum_vec();
    trait_vec();
    alt_strust_vec();
}


