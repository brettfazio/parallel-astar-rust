#![allow(non_snake_case)]

use std::thread;
use std::mem::drop;
use std::collections::{HashSet, BinaryHeap};
use std::sync::mpsc::{channel, Sender, Receiver, TryRecvError};
use std::sync::{Arc, Barrier, Mutex};

mod structs;
use structs::{Incubent, Node, Point, Buffer};

const NUMTHREADS: usize = 8;

const GRAPH: [[i128; 2]; 2] = [
	[1, 9],
	[10, 6]
];

// const GRAPH: [[i32; 10]; 10] = [
// 	[0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
// 	[0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
// 	[0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
// 	[0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
// 	[0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
// 	[0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
// 	[0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
// 	[0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
// 	[0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
// 	[0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
// ];

// Euclidean Distance
fn distance(node: Node, end: Node) -> i128
{
	(((end.position.x - node.position.x).pow(2) + (end.position.y - node.position.y).pow(2)) 
	as f32).sqrt() as i128
}

fn main() 
{
	let mut threads = Vec::with_capacity(NUMTHREADS);
	let mut receivers: Vec<Arc<Mutex<Receiver<Buffer>>>> = Vec::with_capacity(NUMTHREADS);
	let mut transmitters: Vec<Sender<Buffer>> = Vec::with_capacity(NUMTHREADS);
	let mut start = Node::default();
	let end = Node::new(1, 1, 0, 0, 0, Point::default());
	start.h = distance(start, end);
	let barrier = Arc::new(Barrier::new(NUMTHREADS));
	let incubent: Arc<Mutex<Incubent>> = Arc::new(Mutex::new(Incubent::new(start, i128::MAX)));

	// Declares Channels
	for _i in 0..NUMTHREADS
	{
		let (tx, rx) = channel::<Buffer>();
			
	 	transmitters.push(tx.clone());
		receivers.push(Arc::new(Mutex::new(rx)));
	}

	// Here, we would give each thread a different node to start on.
	// Those threads would run a* on each of their respective start nodes.
	for i in 0..NUMTHREADS
	{
		let transmitters = transmitters.clone();
		let incubent = incubent.clone();
		let barrier = barrier.clone();
		let rx = receivers[i].clone();

		// Here we'd pass a start node to each thread.
		threads.push(thread::spawn(move || {
			search(start, i, rx, transmitters, barrier, end, incubent);
		}))
	}

	// Final answer is outputted once all threads are done.
	for thread in threads
	{
		thread.join().expect("Panic");
	}
}

// A* implementation
fn search(start: Node, threadNum: usize, rx: Arc<Mutex<Receiver<Buffer>>>, tx: Vec<Sender<Buffer>>,
	        barrier: Arc<Barrier>,goal_node: Node, incubent: Arc<Mutex<Incubent>>)
{
	let mut buffer: BinaryHeap<Buffer> = BinaryHeap::new();
	let mut open: BinaryHeap<Node> = BinaryHeap::new();
	let mut open_list: HashSet<Node> = HashSet::new();
	let mut closed_list: HashSet<Node> = HashSet::new();
	let mut terminationCondition = false;
	let rx = rx.lock().unwrap();
	
	// Giving appropriate lists start variable.
	open.push(start);
	open_list.insert(start);
	buffer.push(Buffer(start, 0, start));

	loop
	{
		// Loops until we have no more data to add to buffer list.
		loop
		{
			// Receive transmissions until there are none.
			match rx.try_recv()
			{
				Ok(v) =>
				{
					buffer.push(v);	
				},
				Err(error) => 
				{
					terminationCondition = error == TryRecvError::Disconnected;
					break;
				},
			}
		}

		if terminationCondition
		{
			return;
		}

		// Loop until buffer is empty.
		while !buffer.is_empty()
		{
			let Buffer(node, weight, parent) = buffer.pop().unwrap();
			
			// Termination condition (temporary)
			if node.position.x == -1
			{
				return;
			}

			if closed_list.contains(&node)
			{
				if closed_list.get(&node).unwrap().g > weight
				{
					closed_list.remove(&node);
					// Defer Adding to bottom of loop.
				}
				else
				{
					continue;
				}
			}
			else
			{
				if open_list.contains(&node) && open_list.get(&node).unwrap().g <= weight
				{
					continue;
				}
				else
				{
					open_list.remove(&node);
				}
			}

			// Open list is updated with new node values. 
			let mut newNode = Node { g: weight, parent: parent.position, ..node };

			newNode.f = newNode.g + newNode.h;
			open_list.insert(newNode);
			open.push(newNode);
		}
		
		// Safe guards before we check if node is goal node.
		let mut incubentData = incubent.lock().unwrap();

		if open.is_empty() || open.peek().unwrap().g >= incubentData.cost
		{
			continue;
		}
		
		// Process node to see if it's goal node.
		let tempNode = open.pop().unwrap();

		open_list.remove(&tempNode);
		closed_list.insert(tempNode);
		
		// Check if goal node.
		if tempNode == goal_node && incubentData.cost >= tempNode.g
		{
			incubentData.node = tempNode;
			incubentData.cost = tempNode.g;
			//println!("cost to goal node is{}", incubentData.cost);
			println!("We found goal using thread {}", threadNum);

			// Temp fix, let all threads know goal node is found.
			for thread in tx
			{
				thread.send(Buffer(Node::default(), -1, Node::default())).unwrap();
			}
			
			// incubentData is dropped implicitly since scope is left.
			return;
		}

		drop(incubentData);
		
		let adjacent = vec![(-1, 1), (0, 1), (1, 1), (-1, 0), (1, 0), (-1, -1), (0, -1), (1, -1)];

		// First check if valid move, then We will offset to n', and pass off three-tuple
		// to random thread's buffer list.
		for (x, y) in adjacent
		{
			// Safe guard before we test a movement
			if isValidNeighbor(tempNode, x, y)
			{
				// n' is created, now let's put it in a random buffered list.
				let (xCoord, yCoord) = (tempNode.position.x + x, tempNode.position.y + y);
				let nPrime = Node::new(xCoord, yCoord, 0, tempNode.g + GRAPH[xCoord as usize][yCoord as usize] as i128, 0, tempNode.position);
				let mut tried: HashSet<usize> = HashSet::new();
				tried.insert(threadNum);
				
				loop
				{
					let i = computeRecipient(&tried); // calculate random thread to send to
					tried.insert(i);

					match tx[i].send(Buffer(nPrime, GRAPH[nPrime.position.x as usize][nPrime.position.y as usize], tempNode))
					{
						Ok(_) =>
						{
							//println!("From #{} to thread #{}\n\tSending {:?} to thread", threadNum, i, Buffer(nPrime, 1, tempNode));
							break;
						},
						Err(_) =>
						{
							// println!("Error in sending :(\n{:?}", error);
							if tried.len() < NUMTHREADS
							{
								continue;
							}
							else
							{
								break;
							}
						},
					}
				}
			}
		}
	}
}

// NOT ACTUALLY RANDOM, I'M SORRY :(
// Will be random later, :)
fn computeRecipient(setty: &HashSet<usize>) -> usize
{
	let mut index = 0;

	loop
	{
		// Makes sure we don't index the same thread's channel or a dead channel
		if setty.contains(&index)
		{
			index = (index + 1) % NUMTHREADS;
			continue;
		}
		else
		{
			break index;
		}
		
	}
}

// Basic bounds checking
fn isValidNeighbor(node: Node, x: i32, y: i32) -> bool
{
	let (x0, y0) = (node.position.x + x, node.position.y + y);

	x0 >= 0 && y0 >= 0 && x0 < GRAPH.len() as i32 && y0 < GRAPH.len() as i32 && GRAPH[x0 as usize][y0 as usize] != 0
}