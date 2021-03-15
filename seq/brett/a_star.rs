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

pub fn a_star(start: Node, end: Node, graph: &HashMap<Node, Vec<Node>>, h: fn(Node, Node) -> i32) -> HashMap<Node, Node>
{
    let mut heap = BinaryHeap::new();
    heap.push(State { node: start, cost: 0 });

    let mut cost: HashMap<Node, i32> = HashMap::new();
    // Preset cost of all nodes to inf
    let inf = i32::MAX - 1;
    for node in graph.keys()
    {
        cost.insert(*node, inf);
    }
    cost.insert(start, 0);

    let mut came_from: HashMap<Node, Node> = HashMap::new();

    while heap.len() > 0
    {
        let current = heap.pop().unwrap();

        if current.node == end
        {
            break;
        }

        // Compare local cost to cost stored in cost: HashMap<>
        // Can add in h(node, end) since this should not change.
        if current.cost > h(current.node, end) + cost[&current.node]
        {
            // Since this state was originally added, I have found a better route.
            // Skip this one.
            continue;
        }

        let neighbors = &graph[&current.node]; 
        for neighbor in neighbors
        {
            // 1 unit away from neighbors
            let temp_g = cost[&current.node] + 1;

            // neighbor should be a reference so it should work
            if temp_g < cost[&neighbor]
            {
                came_from.insert(*neighbor, current.node);
                
                cost.insert(*neighbor, temp_g);
                let f_score = h(*neighbor, end) + temp_g;
                heap.push(State { node: *neighbor, cost: f_score});
            }
        }

    }

    came_from
}

fn main()
{
    let start = Node {x: 0, y: 0, };

    let one = Node {x: 1, y: 1, };

    let two = Node {x: 2, y: 2, };

    let end = Node {x: 3, y: 3, };

    let mut graph: HashMap<Node, Vec<Node>> = HashMap::new();

    graph.insert(start, vec![one]);
    graph.insert(one, vec![two]);
    graph.insert(two, vec![end]);
    graph.insert(end, vec![]);

    let came_from = a_star(start, end, &graph, heur);

    for a in came_from.keys()
    {
        let b = came_from[a];

        println!("{},{} came from {},{}", a.x, a.y, b.x, b.y);
    }
    println!("Path Length = {}", came_from.len());
}