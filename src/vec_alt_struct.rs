use crate::structs::{Bend, Coordinates, Support};

struct Node {
    coordinates: Coordinates,
    node_type: NodeType
}

impl Node {
    fn coordinates(&self) -> Coordinates {
        self.coordinates.clone()
    }
}

enum NodeType {
    Support(Support),
    Bend(Bend),
}
pub fn alt_strust_vec() {
    let mut vec: Vec<Node> = Vec::new();
    vec.push(Node {
        coordinates: Coordinates::default(),
        node_type: NodeType::Support(Support::default())
    });
    vec.push(Node {
        coordinates: Coordinates::default(),
        node_type: NodeType::Bend(Bend::default())
    });

    println!("\nVector with Alternative Struct:\n");
    for element in vec {
        println!("{:?}", element.coordinates())
    }
}