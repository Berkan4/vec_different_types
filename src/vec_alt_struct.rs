use crate::structs::{Coordinates};

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
    Support(SupportAlt),
    Spring(SpringAlt),
}

#[derive(Default)]
struct SupportAlt {
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
#[derive(Default)]
struct SpringAlt {
    stiffness: f32,
}


pub fn alt_struct_vec() {
    let mut vec: Vec<Node> = Vec::new();
    vec.push(Node {
        coordinates: Coordinates::default(),
        node_type: NodeType::Support(SupportAlt::default())
    });
    vec.push(Node {
        coordinates: Coordinates::default(),
        node_type: NodeType::Spring(SpringAlt::default())
    });

    println!("\nVector with Alternative Struct:\n");
    for element in vec {
        println!("{:?}", element.coordinates())
    }
}