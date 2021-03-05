use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Result};
use std::collections::{BinaryHeap};

#[derive(Eq, Copy, Clone)]
struct Node {
	x: i32,
	y: i32,
	f: i32,
	g: i32
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
		self.f.cmp(&other.f).then_with(|| self.g.cmp(&other.g))
	}
}

impl Display for Node {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "({}, {}) --> {}", self.x, self.y, self.f)
	}
}

impl Node {
	/// Returns Euclidean distance of nodes
	fn h(&self, node: Node) -> f32 {
		(((node.x - self.x).pow(2) + (node.y - self.y).pow(2)) as f32).sqrt()
	}
}

fn main() {
	let mut open: BinaryHeap<Node> = BinaryHeap::new();
	let node = Node { x: 100, y: 0, f: 10, g: 0 };

	open.push(node);

	println!("{}", open.peek().unwrap());
}