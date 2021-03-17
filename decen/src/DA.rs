#![allow(non_snake_case)]

use std::thread;
use std::mem::drop;
use std::collections::{HashSet, BinaryHeap, hash_map};
use hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
// use std::sync::mpsc::{channel, Sender, Receiver};
use std::sync::{Arc, Mutex, atomic};
use atomic::{AtomicU64, Ordering};
use crossbeam::channel::{Sender, Receiver, unbounded};

mod structs;
use structs::{Incumbent, Node, Point, Buffer};

// PascalCase
mod DynamicBarrier;
use DynamicBarrier::DynamicHurdle;
const NUMTHREADS: usize = 8;

// Euclidean Distance
fn distance(node: Node, end: Node) -> i128
{
	(((end.position.x - node.position.x).pow(2) + (end.position.y - node.position.y).pow(2)) 
	as f32).sqrt() as i128
}

pub fn setup(graph: Vec<Vec<char>>) 
{
	let mut threads = Vec::with_capacity(NUMTHREADS);
	let mut receivers: Vec<Receiver<Buffer>> = Vec::with_capacity(NUMTHREADS);
	let mut transmitters: Vec<Sender<Buffer>> = Vec::with_capacity(NUMTHREADS);
	let mut barrier = DynamicHurdle::new(NUMTHREADS);
	let mut startPoint = Point::default();
	let mut endPoint = Point::default();
	let sentMessages = Arc::new(AtomicU64::new(0));
	let receivedMessages = Arc::new(AtomicU64::new(0));

	// Declares Channels
	for _ in 0..NUMTHREADS
	{
		let (tx, rx) = unbounded();
			
	 	transmitters.push(tx);
		receivers.push(rx);
	}

	for i in 0..graph.len()
	{
		for j in 0..graph.len()
		{
			if graph[i][j] == 'S'
			{
				startPoint.x = i as i32;
				startPoint.y = j as i32;
			}
			if graph[i][j] == 'E'
			{
				endPoint.x = i as i32;
				endPoint.y = j as i32;
			}
		}
	}

	let mut start = Node::new(startPoint.x, startPoint.y, 0, 0, 0, Point::default());
	let end = Node::new(endPoint.x, endPoint.y, 0, 0, 0, Point::default());
	start.h = distance(start, end);
	start.f = start.g + start.h;
	let incumbent: Arc<Mutex<Incumbent>> = Arc::new(Mutex::new(Incumbent::new(start, i128::MAX)));

	// Here, we would give each thread a different node to start on.
	// Those threads would run a* on each of their respective start nodes.
	for i in 0..NUMTHREADS
	{
		let transmitters = transmitters.clone();
		let incumbent = incumbent.clone();
		let graph = graph.clone();
		let barrier = barrier.create();
		// let rx = receivers[i].clone();
		let rx = receivers[i].clone();
		let sentMessages = sentMessages.clone();
		let receivedMessages = receivedMessages.clone();

		// Here we'd pass a start node to each thread.
		threads.push(thread::spawn(move || {
			search(start, i, rx, transmitters, barrier, end, incumbent, graph.clone(), sentMessages, receivedMessages);
		}))
	}

	drop(receivers);

	// Final answer is outputted once all threads are done.
	for thread in threads
	{
		thread.join().expect("Panic");
	}

	println!("All threads found goal node.")
}

// A* implementation
fn search(start: Node, threadNum: usize, rx: Receiver<Buffer>, tx: Vec<Sender<Buffer>>,
	        mut barrier: DynamicHurdle, goalNode: Node, incumbent: Arc<Mutex<Incumbent>>, graph: Vec<Vec<char>>,
	        sentMessages: Arc<AtomicU64>, receivedMessages: Arc<AtomicU64>)
{
	let mut buffer: BinaryHeap<Buffer> = BinaryHeap::new();
	let mut open: BinaryHeap<Node> = BinaryHeap::new();
	let mut open_list: HashSet<Node> = HashSet::new();
	let mut closed_list: HashSet<Node> = HashSet::new();
	let mut firstIteration: bool = true;
	let mut tried: HashSet<i32> = HashSet::new();	
	let mut dropBuff: bool = false;
	// Giving appropriate lists start variable.
	open.push(start);
	open_list.insert(start);
	buffer.push(Buffer(start, 0, start));
	tried.insert(threadNum as i32);

	loop
	{
		// Loops until we have no more data to add to buffer list.
		
		barrier.wait();
		
		if !firstIteration && sentMessages.load(Ordering::SeqCst) == receivedMessages.load(Ordering::SeqCst)
		{
			//println!("terminating: {} vs {} in {}", sentMessages.load(Ordering::SeqCst), receivedMessages.load(Ordering::SeqCst), threadNum);

			break;
		}

		barrier.wait();
		//println!("{} vs {} in {}", sentMessages.load(Ordering::SeqCst), receivedMessages.load(Ordering::SeqCst), threadNum);

		firstIteration = false;
		
		loop
		{
			// Receive transmissions until there are none.
			match rx.try_recv()
			{
				Ok(v) =>
				{
					receivedMessages.fetch_add(1, Ordering::SeqCst);
					buffer.push(v);
				},
				Err(_) => break,
			}
		}
		
		if dropBuff
		{
			drop(rx);
			break;
		}

		// Loop until buffer is empty.
		while !buffer.is_empty()
		{
			//println!("buffer list ain't empty ;o");
			let Buffer(node, weight, parent) = buffer.pop().unwrap();

			if closed_list.contains(&node)
			{
				if closed_list.get(&node).unwrap().g > weight
				{
					closed_list.remove(&node);
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
			newNode.h = distance(newNode, goalNode);
			newNode.f = newNode.g + newNode.h;
			open_list.insert(newNode);
			open.push(newNode);
		}

		// Safe guards before we check if node is goal node.
		let mut incumbentData = incumbent.lock().unwrap();
		// All threads fail this check after one returns
		
		// peek returns node with lowest f value

		if open.is_empty() || open.peek().unwrap().f >= incumbentData.cost
		{
			continue;
		}
		
		// Process node to see if it's goal node.
		let tempNode = open.pop().unwrap();

		open_list.remove(&tempNode);
		closed_list.insert(tempNode);
		
		// Check if goal node.
		if tempNode == goalNode && incumbentData.cost >= tempNode.g
		{
			incumbentData.node = tempNode;
			incumbentData.cost = tempNode.g;
			//println!("cost to goal node is {}", incumbentData.cost);

			
			// incumbentData is dropped implicitly since scope is left.
			
			dropBuff = true;
		}
		
		drop(incumbentData);
		
		let adjacent = vec![(-1, 1), (0, 1), (1, 1), (-1, 0), (1, 0), (-1, -1), (0, -1), (1, -1)];
		

		// First check if valid move, then We will offset to n', and pass off three-tuple
		// to random thread's buffer list.
		for (x, y) in adjacent
		{
			// Safe guard before we test a movement
			if isValidNeighbor(&graph, &tempNode, x, y)
			{
				// n' is created, now let's put it in a random buffered list.
				let (xCoord, yCoord) = (tempNode.position.x + x, tempNode.position.y + y);
				let nPrime = Node::new(xCoord, yCoord, 0, tempNode.g + 1, 0, tempNode.position);
				
				loop
				{
					let i = computeRecipient(&nPrime, &tried); // calculate random thread to send to
					
					if i == -1
					{
						buffer.push(Buffer(nPrime, nPrime.g, tempNode));
						break;
					}

					match tx[i as usize].send(Buffer(nPrime, nPrime.g, tempNode))
					{
						Ok(_) =>
						{
							sentMessages.fetch_add(1, Ordering::SeqCst);
							//println!("From #{} to thread #{}\n\tSending {:?} to thread", threadNum, i, Buffer(nPrime, 1, tempNode));
							break;
						},
						Err(_) =>
						{
							//println!("Error in sending :( from {} to {}", threadNum, i);
							tried.insert(i as i32);
							
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

// A non mutually-exlusive function could be the issue
fn computeRecipient(node: &Node, setty: &HashSet<i32>) -> i32
{
	let mut index;
	let hash = calculateHash(&node);

	for i in 0..NUMTHREADS as u64
	{
		index = (hash + i) % NUMTHREADS as u64;
		// Makes sure we don't index the same thread's channel or a dead channel
		if setty.contains(&(index as i32))
		{
			continue;
		}
		else
		{
			return index as i32;
		}
	}
	
	return -1;
}

fn calculateHash<T: Hash>(t: &T) -> u64
{
	let mut state = DefaultHasher::new();

	t.hash(&mut state);
	state.finish()
}

// Basic bounds checking
// Now we care about walls
fn isValidNeighbor(graph: &Vec<Vec<char>>, node: &Node, x: i32, y: i32) -> bool
{
	let (x0, y0) = (node.position.x + x, node.position.y + y);

	x0 >= 0 && y0 >= 0 && x0 < graph.len() as i32 && y0 < graph.len() as i32 && graph[x0 as usize][y0 as usize] != 'W'
}