use grid::Grid;
use std::collections::{BinaryHeap, HashMap};
use std::mem::drop;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::{thread, time};

use rand::Rng;

mod temp_structs;
use temp_structs::{Node, Point};

// Best performance seen with high threading, threads > cores
const NUMTHREADS: usize = 1;

#[derive(PartialEq)]
enum HeurType {
    EuclideanDist,
    Expensive,
    NonAdmissible,
    ExpensiveNonAdmissible,
}

const USING_HEUR: HeurType = HeurType::EuclideanDist;

// const GRAPH: [[char; 10]; 10] = [
//     ['.', '.', '.', '.', '.', '.', 'W', '.', '.', '.'],
//     ['.', '.', '.', '.', 'S', '.', '.', 'W', '.', '.'],
//     ['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
//     ['.', '.', 'W', '.', '.', '.', 'W', '.', '.', '.'],
//     ['W', 'W', '.', '.', '.', 'W', '.', '.', 'W', 'W'],
//     ['.', 'W', '.', 'W', 'W', 'W', 'W', '.', 'W', '.'],
//     ['W', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
//     ['W', '.', 'W', 'W', 'W', '.', 'W', '.', '.', '.'],
//     ['.', '.', '.', '.', 'W', '.', '.', '.', '.', '.'],
//     ['E', '.', 'W', '.', '.', 'W', '.', 'W', '.', '.'],
// ];

fn distance(node: Node, end: Node) -> i128 {
    (((end.position.x - node.position.x).pow(2) + (end.position.y - node.position.y).pow(2)) as f32)
        .sqrt() as i128
}

fn random_wait() {
    //need to import via cargo
    let mut rng = rand::thread_rng();

    let time = rng.gen_range(100..1000);
    let rand_millis = time::Duration::from_millis(time);
    thread::sleep(rand_millis);
}

fn expensive(node: Node, end: Node) -> i128 {
    random_wait();

    distance(node, end)
}

fn non_admissible(node: Node, end: Node, expensive: bool) -> i128 {
    if expensive {
        random_wait();
    }

    let mut rng = rand::thread_rng();

    let percent = rng.gen_range(1.0..100.0);

    let dist = distance(node, end);

    let result = (dist as f64) + (percent / 100.0) * (dist as f64);

    result as i128
}

fn heuristic(node: Node, end: Node) -> i128 {
    if USING_HEUR == HeurType::EuclideanDist {
        return distance(node, end);
    } else if USING_HEUR == HeurType::Expensive {
        return expensive(node, end);
    } else if USING_HEUR == HeurType::NonAdmissible {
        return non_admissible(node, end, false);
    } else if USING_HEUR == HeurType::ExpensiveNonAdmissible {
        return non_admissible(node, end, true);
    }
    // Won't occur
    0
}

fn is_valid(x: usize, y: usize, graph: &Grid<char>) -> bool {
    // Don't need to check below 0 since unsigned
    let in_bounds = x < graph.size().0 && y < graph.size().0;
    if !in_bounds {
        return false;
    }

    return in_bounds && graph[x][y] != 'W';
}

fn search(
    _start: Node,
    id: usize,
    goal_node: Node,
    open: Arc<Mutex<BinaryHeap<Node>>>,
    closed_list: Arc<Mutex<HashMap<Point, Node>>>,
    finished: &AtomicBool,
    graph: Grid<char>,
) {
    loop {
        if finished.load(Ordering::SeqCst) {
            return;
        }
        // wait for open to have node and try getting node
        let mut pq = open.lock().unwrap();

        if pq.len() == 0 {
            continue;
        }

        println!("{}", pq.len());

        let node = pq.pop().unwrap();
        drop(pq);

        // If this is equal to the goal node
        if node.position.x == goal_node.position.x && node.position.y == goal_node.position.y
        {
            println!("found goal! ({},{}).g cost={}", node.position.x, node.position.y, node.g);
            //  Store this and notfiy other threads
            finished.swap(true, Ordering::SeqCst);
            return;
        }

        // Check the closed list
        let mut cl = closed_list.lock().unwrap();
        if cl.contains_key(&node.position) {
            if cl.get(&node.position).unwrap().g < node.g {
                continue;
            }
        }
        cl.insert(node.position, node);
        // Release the lock.
        drop(cl);

        let adjacent = vec![(0, 1), (-1, 0), (1, 0), (0, -1)];

        let mut add_pq = open.lock().unwrap();
        for (x, y) in adjacent {
            let n_x = node.position.x + x;
            let n_y = node.position.y + y;
            if n_x < 0 || n_y < 0 {
                continue;
            }

            if is_valid(n_x as usize, n_y as usize, &graph) {
                // x: i32, y: i32, f: i128, g: i128, h: i128, parent: Point
                let mut n_prime = Node::new(n_x, n_y, 0, node.g + 1, 0, node.position);
                n_prime.h = heuristic(n_prime, goal_node);
                n_prime.f = n_prime.g + n_prime.h;

                // check if closed list contains it
                let mut prime_cl = closed_list.lock().unwrap();
                if prime_cl.contains_key(&n_prime.position) {
                    if prime_cl.get(&n_prime.position).unwrap().g < n_prime.g {
                        continue;
                    }
                }
                prime_cl.insert(n_prime.position, n_prime);
                // Release the lock.
                drop(prime_cl);

                // add to pq
                add_pq.push(n_prime);
            }
        }
        
        drop(add_pq);
        // add_pq goes out of scope here.
    }
}

pub fn setup(graph: Grid<char>) {
    let mut threads = Vec::with_capacity(NUMTHREADS);

    // KPBFS uses global open and close lists
    let open: Arc<Mutex<BinaryHeap<Node>>> = Arc::new(Mutex::new(BinaryHeap::new()));
    let closed_list: Arc<Mutex<HashMap<Point, Node>>> = Arc::new(Mutex::new(HashMap::new()));

    let finished: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));

    let mut startx: i32 = 0;
    let mut starty: i32 = 0;
    let mut endx: i32 = 0;
    let mut endy: i32 = 0;

    for i in 0..graph.size().0 {
        for j in 0..graph.size().0 {
            if graph[i][j] == 'S' {
                startx = i as i32;
                starty = j as i32;
            }
            if graph[i][j] == 'E' {
                endx = i as i32;
                endy = j as i32;
            }
        }
    }

    println!("{} {} {} {}", startx, starty, endx, endy);

    let mut start = Node::new(startx, starty, 0, 0, 0, Point::default());
    let end = Node::new(endx, endy, 0, 0, 0, Point::default());
    start.h = distance(start, end);
    start.f = start.g + start.h;

    // Add to open
    let mut init_open = open.lock().unwrap();
    let mut init_cl = closed_list.lock().unwrap();
    init_open.push(start);
    init_cl.insert(start.position, start);
    drop(init_open);
    drop(init_cl);

    for i in 0..NUMTHREADS {
        let clone_open = Arc::clone(&open);
        let clone_closed_list = Arc::clone(&closed_list);
        let clone_fin = Arc::clone(&finished);
        let graph = graph.clone();
        // Here we'd pass a start node to each thread.
        threads.push(thread::spawn(move || {
            search(
                start,
                i,
                end,
                clone_open,
                clone_closed_list,
                &clone_fin,
                graph,
            );
        }))
    }

    // Final answer is outputted once all threads are done.
    for thread in threads {
        thread.join().expect("Panic");
    }
}
