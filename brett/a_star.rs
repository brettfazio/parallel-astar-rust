use std::collections::HashMap;

pub struct Node
{
    x: i32,
    y: i32,
}

// Manhattan Distance as heuristic
pub fn heur(current: Node, target: Node) -> i32
{
    let result = i32::abs(current.x - target.x) + i32::abs(current.y - target.y);

    result
}

pub fn a_star(start: Node, end: Node, graph: &[HashMap<Node, &[Node]>], h: fn(Node, Node) -> i32)
{

}

fn main()
{

    let start = Node {
        x: 0,
        y: 0,
    };

    let end = Node {
        x: 0,
        y: 0,
    };

    println!("Hello");
}
