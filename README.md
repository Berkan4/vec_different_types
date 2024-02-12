# vec_different_types
This is a playground showcasing 2 ways to create a vector holding elements of different types
## Initial situation
Assume you have two structs `Support` and `Bend` that share common behaviours like having coordinates in space with the struct `Coordinates`.
```rust
struct Support {
    coordinates: Coordinates,
    // Other `Support` specific fields
}

struct Bend {
    coordinates: Coordinates,
    // Other `Bend` specific fields
}

struct Coordinates {
    x: f32,
    y: f32,
    z: f32,
}
```
In order to create a vector that can hold either type and access the shared behaviour we have the following two approaches.
### Using Enum
Create a enum containing tuple-like variants.
```rust
enum NodeEnum {
    Support(Support),
    Bend(Bend),
}
```
Implement functions with desired behaviour via pattern matching for this enum
```rust
impl NodeEnum {
    fn coordinates(&self) -> Coordinates {
        match self {
            NodeEnum::Support(support) => support.coordinates.clone(),
            NodeEnum::Bend(bend) => bend.coordinates.clone(),
        }
    }
}
```
Create a vector of the type `Vec<NodeEnum>` and push elements
```rust
pub fn enum_vec() {
    let mut vec: Vec<NodeEnum> = Vec::new();
    vec.push(NodeEnum::Support(Support::default()));
    vec.push(NodeEnum::Bend(Bend::default()));

    println!("\nVector with Enum:\n");
    for element in vec {
        // Access shared behaviour
        println!("{:?}", element.coordinates())
    }
}
```
### Using Trait
Create a trait `NodeTrait` with a prototype function of the desired behaviour
```rust
trait NodeTrait {
    fn coordinates(&self) -> Coordinates;
}
```
Implement the trait and functions with a body for the structs
```rust
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
```
Create a vector of type `Vec<Box<dyn Trait>>` and push elements
```rust
pub fn trait_vec() {
    let mut vec: Vec<Box<dyn NodeTrait>> = Vec::new();
    vec.push(Box::new(Support::default()));
    vec.push(Box::new(Bend::default()));

    println!("\nVector with Box<dyn Trait>:\n");
    for element in vec {
        // Access shared behaviour
        println!("{:?}", element.coordinates())
    }
}
```
### Using an alternative struct
Create a struct `Node` with a field `node_type`
```rust
struct Node {
    coordinates: Coordinates,
    node_type: NodeType
}
```
Add an enum as a replacement for the structs `Support` and `Bend`
```rust
enum NodeType {
    Support(Support),
    Bend(Bend),
}
```
Implement the function with the shared behaviour
```rust
impl Node {
    fn coordinates(&self) -> Coordinates {
        self.coordinates.clone()
    }
}
```


