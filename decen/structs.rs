use std::cmp::{Ordering};

// Goal node path and cost of path.
pub struct Incubent
{
	pub node: Node,
	pub cost: i128
}

impl Incubent
{
	pub fn new (node: Node, cost: i128) -> Incubent
	{
		Incubent { node, cost }
	}
}

// Container for trasmitting messages.
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct Buffer (pub Node, pub i128, pub Node);

impl Ord for Buffer
{
	fn cmp(&self, other: &Self) -> Ordering 
	{
		other.0.f.cmp(&self.0.f)
	}
}

impl PartialOrd for Buffer
{
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		Some(self.cmp(other))
	}
}

// Point struct for node coordinates.
#[derive(Eq, Hash, Clone, Copy, Debug)]
pub struct Point
{
	pub x: i32,
	pub y: i32,
}

impl Point
{
	pub fn new(x: i32, y: i32) -> Point
	{
		Point { x, y }
	}
}

impl PartialEq for Point
{
	fn eq(&self, other: &Self) -> bool 
	{
		self.x == other.x && self.y == other.y
	}
}				

#[derive(Eq, Hash, Clone, Copy, Debug)]
pub struct Node
{
	pub position: Point,
	pub f: i128,
	pub g: i128,
	pub h: i128,
	pub parent: Point,
}

impl Node
{
	pub fn new(x: i32, y: i32, f: i128, g: i128, h: i128, parent: Point) -> Node
	{
		Node
		{
			position: Point { x, y }, f, g, h, parent
		}
	}
}

// TODO: Change for tuple
impl Ord for Node
{
	fn cmp(&self, other: &Self) -> Ordering 
	{
		other.f.cmp(&self.f)
	}
}


impl PartialOrd for Node
{
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		Some(self.cmp(other))
	}
}

impl PartialEq for Node
{
	fn eq(&self, other: &Self) -> bool 
	{
		self.position == other.position
	}
}