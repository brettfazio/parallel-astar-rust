use std::{thread, time};
use std::mem::drop;
use std::collections::{HashSet, BinaryHeap};
use std::sync::mpsc::{channel, Sender, Receiver, TryRecvError};
use std::sync::{Arc, Barrier, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};

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

const GRAPH: [[char; 0]; 0] = [];

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

fn is_valid(x: usize, y: usize) -> bool
{
    // Don't need to check below 0 since unsigned
    let in_bounds = x < GRAPH.len() && y < GRAPH.len();
    if !in_bounds
    {
        return false;
    }

    return in_bounds && GRAPH[x][y] != 'W'
}

fn search(start: Node,
    id: usize,
    goal_node: Node,
    open: Arc<Mutex<BinaryHeap<Node>>>,
    open_list: Arc<Mutex<HashSet<Node>>>,
    closed_list: Arc<Mutex<HashSet<Node>>>,
    finished: &AtomicBool)
{
    loop {
        if finished.load(Ordering::Relaxed)
        {
            return;
        }

        // wait for open to have node and try getting node
        let mut pq = open.lock().unwrap();

        if pq.len() == 0
        {
            continue;
        }

        let node = pq.pop().unwrap();
        drop(pq);
        // If this is equal to the goal node
        if node == goal_node
        {
            //  Store this and notfiy other threads
            finished.swap(true, Ordering::Relaxed);
            return;
        }

        // Check the closed list
        let mut cl = closed_list.lock().unwrap();
        if cl.contains(&node)
        {
            if cl.get(&node).unwrap().g > node.g
            {
                cl.remove(&node);
            }
            else
            {
                continue;
            }
        }
        // Release the lock.
        drop(cl);

        

        let adjacent = vec![(0, 1), (-1, 0), (1, 0), (0, -1)];

        let mut add_pq = open.lock().unwrap();
        for (x, y) in adjacent
		{
            let n_x = node.position.x + x;
            let n_y = node.position.y + y;

            if n_x < 0 || n_y < 0
            {
                continue;
            }

            if is_valid(n_x as usize, n_y as usize)
            {
                // x: i32, y: i32, f: i128, g: i128, h: i128, parent: Point
                let mut n_prime = Node::new(n_x, n_y, 0, node.g + 1, 0, node.position);
                n_prime.h = heuristic(n_prime, goal_node);
                n_prime.f = n_prime.g + n_prime.h;

                // check if closed list contains it
                let mut prime_cl = closed_list.lock().unwrap();
                if prime_cl.contains(&n_prime)
                {
                    if prime_cl.get(&n_prime).unwrap().g > n_prime.g
                    {
                        prime_cl.remove(&n_prime);
                    }
                    else
                    {
                        continue;
                    }
                }
                // Release the lock.
                drop(prime_cl);


                // do same for open list
                let mut prime_ol = open_list.lock().unwrap();
                if prime_ol.contains(&n_prime)
                {
                    if prime_ol.get(&n_prime).unwrap().g > n_prime.g
                    {
                        prime_ol.remove(&n_prime);
                    }
                    else
                    {
                        continue;
                    }
                }

                // add to open list
                prime_ol.insert(n_prime);

                // Release the lock.
                drop(prime_ol);

                
                // add to pq
                add_pq.push(n_prime);

            }
        }

        // add_pq goes out of scope here.
    }
    
}

pub fn setup()
{

    let mut threads = Vec::with_capacity(NUMTHREADS);

    // KPBFS uses global open and close lists
    let mut open: Arc<Mutex<BinaryHeap<Node>>> = Arc::new(Mutex::new(BinaryHeap::new()));
	let mut open_list: Arc<Mutex<HashSet<Node>>> = Arc::new(Mutex::new(HashSet::new()));
	let mut closed_list: Arc<Mutex<HashSet<Node>>> = Arc::new(Mutex::new(HashSet::new()));

    let mut finished: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));

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
        let clone_fin = Arc::clone(&finished);
		// Here we'd pass a start node to each thread.
		threads.push(thread::spawn(move || {
			search(start, i, end, clone_open, clone_open_list, clone_closed_list, &clone_fin);
		}))
	}
}