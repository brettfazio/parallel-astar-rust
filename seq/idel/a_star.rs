use std::cmp::{Ordering};
use std::collections::{BinaryHeap, HashSet};

#[derive(Hash, Eq, Copy, Clone, Debug)]
struct Node {
	x: i32,
	y: i32,
	f: i32,
	g: i32,
	h: i32
}

impl PartialOrd for Node {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl PartialEq for Node {
	fn eq(&self, other: &Self) -> bool {
		self.x == other.x && self.y == other.y
	}
}

impl Ord for Node {
	fn cmp(&self, other: &Self) -> Ordering {
		other.f.cmp(&self.f)
	}
}

impl Node {
	/// Returns Euclidean distance of nodes
	fn new(x: i32, y: i32) -> Node {
		Node { x, y, f: 0, g: 0, h: 0 }
	}
}

fn distance(node: Node, end: Node) -> i32 {
	(((end.x - node.x).pow(2) + (end.y - node.y).pow(2)) as f32).sqrt() as i32
}

fn path_finding(start: Node, end: Node, graph: [[Option<i32>; 10]; 10], _path: Vec<i32>) -> bool {
	let mut open: BinaryHeap<Node> = BinaryHeap::new();
	let mut open_list: HashSet<Node> = HashSet::new();
	let mut closed_list: HashSet<Node> = HashSet::new();
	let adjacent = [(-1, 1), (0, 1), (1, 1), (-1, 0), (1, 0), (-1, -1), (0, -1), (1, -1)];

	open.push(start);
	open_list.insert(start);


	while !open.is_empty() {
		let node = open.pop().unwrap();

		println!("Node: {:?}", node);

		if node.x == end.x && node.y == end.y {
			return true;
		}

		println!("Neighbors");
		for neighbor in adjacent.iter() {
			let mut next = Node::new(node.x + neighbor.0, node.y + neighbor.1);

			if next == end {
				// correctly set path taken
				return true;
			}
		
			// Check if in range and not blocked
			if next.x < 0 || next.y < 0 || next.x as usize >= graph.len() || next.y as usize >= graph[next.x as usize].len()
				|| graph[next.y as usize][next.x as usize] == None {
				continue;
			}

			println!("\tNeighbor: {:?}", next);

			match open_list.get(&next) {
				Some(node) => if node.f < next.f { continue },
				None => {}
			}

			match closed_list.get(&next) {
				Some(node) => if node.f < next.f { closed_list.remove(&next); continue },
				None => {}
			}

			next.g = next.g + 1;
			next.h = distance(next, end);
			next.f = next.g + next.h;
			
			open.push(next);
			open_list.insert(next);
		}

		open_list.remove(&node);
		closed_list.insert(node);
	}

	false
}

fn main() {
	let start = Node::new(0, 0);
	let end = Node::new(9, 9);
	let path_taken = vec!();
	let graph: [[Option<i32>; 10]; 10] = [
		[Some(0), Some(0), None,    Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0)],
		[Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0)],
		[Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0)],
		[Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0)],
		[Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0)],
		[Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0)],
		[Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0)],
		[Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0)],
		[Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0)],
		[Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0)]
	];

	println!("Was the path found? {}", path_finding(start, end, graph, path_taken));
}