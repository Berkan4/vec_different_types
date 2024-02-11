use crate::structs::{Bend, Coordinates, Support};
trait NodeTrait {
    fn coordinates(&self) -> Coordinates;
}

impl NodeTrait for Support {
    fn coordinates(&self) -> Coordinates {
        self.coordinates.clone()
    }
}

impl NodeTrait for Bend {
    fn coordinates(&self) -> Coordinates {
        self.coordinates.clone()
    }
}

pub fn trait_vec() {
    let mut vec: Vec<Box<dyn NodeTrait>> = Vec::new();
    vec.push(Box::new(Support::default()));
    vec.push(Box::new(Bend::default()));

    println!("\nVector with Box<dyn Trait>:\n");
    for element in vec {
        println!("{:?}", element.coordinates())
    }
}
