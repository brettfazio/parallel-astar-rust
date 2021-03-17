use std::default::Default;
use std::cmp::Ordering;

// Goal node path and cost of path.
pub struct Incumbent
{
	pub node: Node,
	pub cost: i128
}

impl Incumbent
{
	pub fn new(node: Node, cost: i128) -> Incumbent
	{
		Incumbent { node, cost }
	}
}

// Container for transmitting messages.
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
#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct Point
{
	pub x: i32,
	pub y: i32
}

/*
impl Point
{
	pub fn new(x: i32, y: i32) -> Point
	{
		Point { x, y }
	}
}
*/

impl Default for Point
{
	fn default() -> Point
	{
		Point { x: -1, y: -1 }
	}
}		

#[derive(Eq, Hash, Clone, Copy, Default, Debug)]
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