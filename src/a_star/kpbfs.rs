use std::{
    thread,
    mem::drop,
    collections::{HashMap, BinaryHeap},
    sync::{Arc, Mutex},
    sync::atomic::{AtomicBool, Ordering}
};
use super::utils::{
	structs::{Node, Point, Flags},
    helpers
};

// Best performance seen with high threading, threads > cores

fn search(
    _start: Node,
    id: usize,
    goal_node: Node,
    open: Arc<Mutex<BinaryHeap<Node>>>,
    closed_list: Arc<Mutex<HashMap<Point, Node>>>,
    finished: &AtomicBool,
    graph: Vec<Vec<char>>,
    flags: Flags,
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

        let node = pq.pop().unwrap();
        drop(pq);

        //println!("{},{} {}", node.position.x, node.position.y, id);

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

        //println!("{},{} g={}", node.position.x, node.position.y, node.g);

        let adjacent = vec![(0, 1), (-1, 0), (1, 0), (0, -1)];

        for (x, y) in adjacent {
            let n_x = node.position.x + x;
            let n_y = node.position.y + y;
            if n_x < 0 || n_y < 0 {
                continue;
            }

            if helpers::is_valid_neighbor(&graph, &node, x, y) {
            //if is_valid(n_x as usize, n_y as usize, &graph) {
                // x: i32, y: i32, f: i128, g: i128, h: i128, parent: Point
                let mut n_prime = Node::new(n_x, n_y, 0, node.g + 1, 0, node.position);
                n_prime.h = helpers::heuristic(n_prime, goal_node, &flags.heur);
                n_prime.f = n_prime.g + n_prime.h;

                // check if closed list contains it
                let mut prime_cl = closed_list.lock().unwrap();
                if prime_cl.contains_key(&n_prime.position) {
                    if prime_cl.get(&n_prime.position).unwrap().g <= n_prime.g {
                        continue;
                    }
                }
                prime_cl.insert(n_prime.position, n_prime);
                // Release the lock.
                drop(prime_cl);

                // add to pq
                let mut add_pq = open.lock().unwrap();
                add_pq.push(n_prime);
                drop(add_pq);
                // add_pq goes out of scope here.
            }
        }
        
        
    }
}

pub fn setup(start_point: Point, end_point: Point, flags: Flags) {
    let Flags { heur, graph, threads: thread_cnt } = flags;
    let mut threads = Vec::with_capacity(thread_cnt);

    // KPBFS uses global open and close lists
    let open: Arc<Mutex<BinaryHeap<Node>>> = Arc::new(Mutex::new(BinaryHeap::new()));
    let closed_list: Arc<Mutex<HashMap<Point, Node>>> = Arc::new(Mutex::new(HashMap::new()));

    let finished: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));

    let mut start = Node::new(start_point.x, start_point.y, 0, 0, 0, Point::default());
    let end = Node::new(end_point.x, end_point.y, 0, 0, 0, Point::default());
    start.h = helpers::heuristic(start, end, &flags.heur);
    start.f = start.g + start.h;

    // Add to open
    let mut init_open = open.lock().unwrap();
    let mut init_cl = closed_list.lock().unwrap();
    init_open.push(start);
    init_cl.insert(start.position, start);
    drop(init_open);
    drop(init_cl);

    for i in 0..thread_cnt {
        let clone_open = Arc::clone(&open);
        let clone_closed_list = Arc::clone(&closed_list);
        let clone_fin = Arc::clone(&finished);
        let graph = graph.clone();
        let flags = Flags { graph: graph.clone(), heur, threads: thread_cnt };

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
                flags,
            );
        }))
    }

    // Final answer is outputted once all threads are done.
    for thread in threads {
        thread.join().expect("Panic");
    }
}
