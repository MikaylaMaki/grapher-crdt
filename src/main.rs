use std::rc::Rc;

#[derive(Hash, Debug)]
struct Edge {
    from: Rc<Node>,
    to: Rc<Node>,
}

impl Edge {
    fn new(from: Rc<Node>, to: Rc<Node>) {
        Edge {
            from: Rc::clone(&from),
            to: Rc::clone(&to)
        }
    }
}

#[derive(Debug)]
enum NodeValue {
    Number,
    String,
}

struct Node {
    id: usize,
    value: NodeValue,
    edges: Vec<Rc<Edge>>,
}

impl Node {
    fn new(id: usize, value: NodeValue) -> {
        Node {
            id,
            value,
            edges: vec![]
        }
    }
}

struct Graph {
    nodes: Vec<Rc<Node>>,
}

impl Graph {
    fn new() -> Graph {
        Graph { nodes: vec![] }
    }
}

impl fmt::Debug for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for node in self.nodes.into_iter() {
            fmt.write_fmt("Node: {} ({})", node.id, node.value);
            fmt.write_fmt("Edges:");
            fmt.debug_list().entries(node.edges.into_iter());
            fmt.write_fmt("");
        }
        fmt.finish();
    }
}

fn main() {
    let node1 = Node::new(1, Int);
    let node2 = Node::new(2, String);
    let edge =

    println!("Hello, world!");
}
