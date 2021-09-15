// use std::rc::Rc;
//
// #[derive(Hash, Debug, Display)]
// struct Edge {
//     from: Rc<Node>,
//     to: Rc<Node>,
// }
//
// impl Edge {
//     fn new(from: Rc<Node>, to: Rc<Node>) {
//         Edge {
//             from: Rc::clone(&from),
//             to: Rc::clone(&to)
//         }
//     }
// }
//
// #[derive(Debug, Display)]
// enum NodeValue {
//     Number,
//     String,
// }
//
// struct Node {
//     id: usize,
//     value: NodeValue,
//     edges: Vec<Rc<Edge>>,
// }
//
// impl Node {
//     fn new(id: usize, value: NodeValue) -> {
//         Node {
//             id,
//             value,
//             edges: vec![]
//         }
//     }
// }
//
// struct Graph {
//     nodes: Vec<Rc<Node>>,
// }
//
// impl Graph {
//     fn new() -> Graph {
//         Graph { nodes: vec![] }
//     }
// }
//
// impl fmt::Debug for Graph {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         for node in self.nodes.into_iter() {
//             fmt.write_fmt("Node: {} ({})", node.id, node.value);
//             fmt.write_fmt("Edges:");
//             fmt.debug_list().entries(node.edges.into_iter());
//             fmt.write_fmt("");
//         }
//         fmt.finish();
//     }
// }
//
// fn main() {
//     let node1 = Node::new(1, Int);
//     let node2 = Node::new(2, String);
//     let edge =
//
//     println!("Hello, world!");
// }

fn main() {
    let immutable_box = Box::new(5u32);

    println!("immutable_box contains {}", immutable_box);

    // Mutability error
    //*immutable_box = 4;

    // *Move* the box, changing the ownership (and mutability)
    let mut mutable_box = immutable_box;

    println!("mutable_box contains {}", mutable_box);

    // Modify the contents of the box
    *mutable_box = 4;

    println!("mutable_box now contains {}", mutable_box);
    main2();
}

// This function takes ownership of a box and destroys it
fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("Destroying box that contains {}", boxed_i32);
}

// This function borrows an i32
fn borrow_i32(borrowed_i32: &i32) {
    //Declaration of what's requested
    println!("This int is: {}", borrowed_i32);
}

fn main2() {
    // Create a boxed i32, and a stacked i32
    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    // Borrow the contents of the box. Ownership is not taken,
    // so the contents can be borrowed again.
    borrow_i32(&boxed_i32); //Actually getting the thing that is requested
    borrow_i32(&stacked_i32);

    {
        // Take a reference to the data contained inside the box
        let _ref_to_i32: &i32 = &boxed_i32;

        // Error!
        // Can't destroy `boxed_i32` while the inner value is borrowed later in scope.
        // eat_box_i32(boxed_i32);
        // FIXME ^ Comment out this line

        // Attempt to borrow `_ref_to_i32` after inner value is destroyed
        borrow_i32(_ref_to_i32);
        // `_ref_to_i32` goes out of scope and is no longer borrowed.
    }

    // `boxed_i32` can now give up ownership to `eat_box` and be destroyed
    eat_box_i32(boxed_i32);
}
