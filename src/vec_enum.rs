use crate::structs::{Bend, Coordinates, Support};

enum NodeEnum {
    Support(Support),
    Bend(Bend),
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
    vec.push(NodeEnum::Bend(Bend::default()));

    println!("\nVector with Enum:\n");
    for element in vec {
        println!("{:?}", element.coordinates())
    }
}