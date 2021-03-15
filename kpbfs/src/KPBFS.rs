use std::{thread, time};
use std::mem::drop;
use std::collections::{HashSet, BinaryHeap};
use std::sync::mpsc::{channel, Sender, Receiver, TryRecvError};
use std::sync::{Arc, Barrier, Mutex};

use rand::Rng;

mod temp_structs;
use temp_structs::{Incubent, Node, Point, Buffer};

// Best performance seen with high threading, threads > cores
const NUMTHREADS: usize = 32;

#[derive(PartialEq)]
enum HeurType
{
    EuclideanDist, Expensive, NonAdmissible, ExpensiveNonAdmissible
}

const USING_HEUR: HeurType = HeurType::Expensive;

fn distance(node: Node, end: Node) -> i128
{
	(((end.position.x - node.position.x).pow(2) + (end.position.y - node.position.y).pow(2)) 
	        as f32).sqrt() as i128
}

fn random_wait()
{
    //need to import via cargo
    let mut rng = rand::thread_rng();

    let time = rng.gen_range(100..1000);
    
    let rand_millis = time::Duration::from_millis(time);
    thread::sleep(rand_millis);
}

fn expensive(node: Node, end: Node) -> i128
{
    random_wait();

	distance(node, end)
}

fn non_admissible(node: Node, end: Node, expensive: bool) -> i128
{
    if expensive
    {
        random_wait();
    }

    let mut rng = rand::thread_rng();

    let percent = rng.gen_range(1.0..100.0);


	let dist = distance(node, end);

    let result = (dist as f64) + (percent/100.0)* (dist as f64);

    result as i128
}

fn heuristic(node: Node, end: Node) -> i128
{
    if USING_HEUR == HeurType::EuclideanDist
    {
        return distance(node, end);
    }
    else if USING_HEUR == HeurType::Expensive
    {
        return expensive(node, end);
    }
    else if USING_HEUR == HeurType::NonAdmissible
    {
        return non_admissible(node, end, false);
    }
    else if USING_HEUR == HeurType::ExpensiveNonAdmissible
    {
        return non_admissible(node, end, true);
    }
    
    // Won't occur
    0
}

fn search(start: Node,
    id: usize,
    goal_node: Node,
    open: Arc<Mutex<BinaryHeap<Node>>>,
    open_list: Arc<Mutex<HashSet<Node>>>,
    closed_list: Arc<Mutex<HashSet<Node>>>)
{
    loop {
        let len = open_list.lock().and_then(|list | Ok(list.len()));
        if let Ok(l) = len {
            if l == 0 {
                return;
            }
        } else {
            return;
        }

        // wait for open to have node and try getting node
        if let Ok(mut pq) = open.lock() {
            let node = pq.pop();
        } else {
            return;
        }

        // expand node
        
        // update

        // check for answer
    }
    
}

pub fn setup()
{

    let mut threads = Vec::with_capacity(NUMTHREADS);

    // KPBFS uses global open and close lists
    let mut open: Arc<Mutex<BinaryHeap<Node>>> = Arc::new(Mutex::new(BinaryHeap::new()));
	let mut open_list: Arc<Mutex<HashSet<Node>>> = Arc::new(Mutex::new(HashSet::new()));
	let mut closed_list: Arc<Mutex<HashSet<Node>>> = Arc::new(Mutex::new(HashSet::new()));

    let mut start = Node::default();
    let end = Node::new(1, 1, 0, 0, 0, Point::default());
    start.h = distance(start, end);

    // Here, we would give each thread a different node to start on.
	// Those threads would run a* on each of their respective start nodes.
	for i in 0..NUMTHREADS
	{
        let clone_open = Arc::clone(&open);
        let clone_open_list = Arc::clone(&open_list);
        let clone_closed_list = Arc::clone(&closed_list);
		// Here we'd pass a start node to each thread.
		threads.push(thread::spawn(move || {
            
			search(start, i, end, clone_open, clone_open_list, clone_closed_list);
		}))
	}
}