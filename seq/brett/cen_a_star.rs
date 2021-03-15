use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::hash::Hash;

// Nodes with heuristic metadata.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Node
{
    x: i32, // X cartesian location
    y: i32, // Y cartesian location
    // f: i32, // f = g + h
    // g: i32, // g = cost to get here to far
    // h: i32, // h = estimated hueristic remaining
}

impl Ord for Node
{
    fn cmp(&self, other: &Self) -> Ordering
    {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.x.cmp(&self.x)
            .then_with(|| self.y.cmp(&other.y))
    }
}

impl PartialOrd for Node
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        Some(self.cmp(other))
    }
}

// States in graph search
#[derive(Copy, Clone, Eq, PartialEq)]
struct State
{
    node: Node,
    cost: i32,
}

impl Ord for State
{
    fn cmp(&self, other: &Self) -> Ordering
    {
        other.cost.cmp(&self.cost)
            .then_with(|| self.node.x.cmp(&other.node.x))
            .then_with(|| self.node.y.cmp(&other.node.y))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        Some(self.cmp(other))
    }
}

// Manhattan Distance as heuristic
pub fn heur(current: Node, target: Node) -> i32
{
    let result = i32::abs(current.x - target.x) + i32::abs(current.y - target.y);

    result
}

pub fn cen_a_star(start: Node, end: Node, graph: &HashMap<Node, Vec<Node>>, h: fn(Node, Node) -> i32) -> HashMap<Node, Node>
{

}

fn main()
{


}