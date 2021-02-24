use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::rc::Rc;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Node
{
    x: i32, // X cartesian location
    y: i32, // Y cartesian location
    f: i32, // f = g + h
    g: i32, // g = cost to get here to far
    h: i32, // h = estimated hueristic remaining
}

// Ordering for min heap
impl Ord for Node
{
    fn cmp(&self, other: &Self) -> Ordering
    {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.f.cmp(&self.f)
            .then_with(|| self.g.cmp(&other.g))
            .then_with(|| self.h.cmp(&other.h))
            .then_with(|| self.x.cmp(&other.x))
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


// Manhattan Distance as heuristic
pub fn heur(current: Node, target: Node) -> i32
{
    let result = i32::abs(current.x - target.x) + i32::abs(current.y - target.y);

    result
}

pub fn a_star(mut start: Node, end: Node, graph: HashMap<Node, [Node]>, h: fn(Node, Node) -> i32)
{

    let mut heap = BinaryHeap::new();

    start.g = 0;
    start.h = heur(start, end);

    heap.push(start);

    let mut cameFrom: HashMap<Node, Node> = HashMap::new();

    while heap.len() > 0
    {
        let current = heap.pop().unwrap();

        if current == end
        {
            break;
        }

        for neighbor in &mut graph[&current]
        {
            // 1 unit away from neighbors
            let temp_g = current.g + 1;

            // neighbor should be a reference so it should work
            if temp_g < neighbor.g
            {
                cameFrom[neighbor] = current;
                

                neighbor.g = temp_g;
                neighbor.f = heur(*neighbor, end);
            }
        }

    }

}

fn main()
{

    let mut start = Node {
        x: 0,
        y: 0,
        f: 0,
        g: 0,
        h: 0,
    };

    let mut end = Node {
        x: 0,
        y: 0,
        f: 0,
        g: 0,
        h: 0,
    };

    println!("Hello");
}
