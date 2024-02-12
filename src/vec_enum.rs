use crate::structs::{Spring, Coordinates, Support};

enum NodeEnum {
    Support(Support),
    Bend(Spring),
}

impl NodeEnum {
    fn coordinates(&self) -> Coordinates {
        match self {
            NodeEnum::Support(support) => support.coordinates.clone(),
            NodeEnum::Bend(bend) => bend.coordinates.clone(),
        }
    }
}

pub fn enum_vec() {
    let mut vec: Vec<NodeEnum> = Vec::new();
    vec.push(NodeEnum::Support(Support::default()));
    vec.push(NodeEnum::Bend(Spring::default()));

    println!("\nVector with Enum:\n");
    for element in vec {
        println!("{:?}", element.coordinates())
    }
}