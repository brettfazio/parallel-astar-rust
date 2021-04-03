use atomic::{AtomicU64, Ordering};
use crossbeam::channel::{Sender, Receiver, unbounded};
use std::{
    thread,
    mem::drop,
    collections::{HashSet, BinaryHeap},
    sync::{Arc, Mutex, atomic}
};
use super::utils::{
    structs::{Incumbent, Node, Point, Buffer, Flags},
    dynamic_barrier::DynamicHurdle,
    helpers
};

const NUM_THREADS: usize = 8;

pub fn setup(start_point: Point, end_point: Point, flags: Flags)  {
    let Flags { heur, graph, threads: thread_cnt } = flags;
    let mut threads = Vec::with_capacity(NUM_THREADS);
    let mut receivers: Vec<Receiver<Buffer>> = Vec::with_capacity(NUM_THREADS);
    let mut transmitters: Vec<Sender<Buffer>> = Vec::with_capacity(NUM_THREADS);
    let mut barrier = DynamicHurdle::new(NUM_THREADS);
    let sent_messages = Arc::new(AtomicU64::new(0));
    let received_messages = Arc::new(AtomicU64::new(0));

    // Declares channels
    for _ in 0..NUM_THREADS {
        let (tx, rx) = unbounded();
            
        transmitters.push(tx);
        receivers.push(rx);
    }

    let mut start = Node::new(start_point.x, start_point.y, 0, 0, 0, Point::default());
    let end = Node::new(end_point.x, end_point.y, 0, 0, 0, Point::default());
    start.h = helpers::heuristic(start, end, &flags.heur);
    start.f = start.g + start.h;
    let incumbent: Arc<Mutex<Incumbent>> = Arc::new(Mutex::new(Incumbent::new(start, i128::MAX)));

    // Here, we would give each thread a different node to start on.
    // Those threads would run A* on each of their respective start nodes.
    for i in 0..NUM_THREADS {
        let transmitters = transmitters.clone();
        let incumbent = incumbent.clone();
        let graph = graph.clone();
        let barrier = barrier.create();
        let rx = receivers[i].clone();
        let sent_messages = sent_messages.clone();
        let received_messages = received_messages.clone();
        let flags = Flags { graph: graph.clone(), heur, threads: thread_cnt };

        // Here we'd pass a start node to each thread.
        threads.push(thread::spawn(move || {
            search(start, i, rx, transmitters, barrier, end, incumbent, graph.clone(), sent_messages, received_messages, flags);
        }))
    }

    // When receiver is cloned, an instance of it remains alive in main.
    // We drop so that only the single clone has a reference to the receiver.
    drop(receivers);

    // Final answer is outputted once all threads are done.
    for thread in threads {
        thread.join().expect("Panic");
    }

    println!("All threads found goal node.")
}

// A* implementation
fn search(start: Node, thread_num: usize, rx: Receiver<Buffer>, tx: Vec<Sender<Buffer>>,
          mut barrier: DynamicHurdle, goal_node: Node, incumbent: Arc<Mutex<Incumbent>>, graph: Vec<Vec<char>>,
          sent_messages: Arc<AtomicU64>, received_messages: Arc<AtomicU64>,
          flags: Flags) {
    let mut buffer: BinaryHeap<Buffer> = BinaryHeap::new();
    let mut closed_list: HashSet<Node> = HashSet::new();
    let mut open: BinaryHeap<Node> = BinaryHeap::new();
    let mut open_list: HashSet<Node> = HashSet::new();
    let mut tried: HashSet<i32> = HashSet::new();	
    let mut first_iteration: bool = true;
    let mut exit: bool = false;
    
    // Giving appropriate lists start variable.
    open.push(start);
    open_list.insert(start);
    buffer.push(Buffer(start, 0, start));
    tried.insert(thread_num as i32);

    loop {
        // Initial thread synchronization before checking for count and messages.
        barrier.wait();
        
        if !first_iteration && sent_messages.load(Ordering::SeqCst) == received_messages.load(Ordering::SeqCst) {			
            break;
        }
        
        // Barrier wait forces all threads to read the same d_me count.
        barrier.wait();
        first_iteration = false;

        // Loops until we have no more data to add to buffer list (no more messages received).
        loop {
            match rx.try_recv() {
                Ok(v) => {
                    received_messages.fetch_add(1, Ordering::SeqCst);
                    buffer.push(v);
                },
                Err(_) => break,
            }
        }
        
        // Receiver and barrier are implicitely dropped, no need to drop them.
        if exit {
            break;
        }

        // Loop until buffer is empty.
        while !buffer.is_empty() {
            let Buffer(node, weight, parent) = buffer.pop().unwrap();

            if closed_list.contains(&node) {
                if closed_list.get(&node).unwrap().g > weight {
                    closed_list.remove(&node);
                }
                else {
                    continue;
                }
            }
            else {
                if open_list.contains(&node) && open_list.get(&node).unwrap().g <= weight {
                    continue;
                }
                else {
                    open_list.remove(&node);
                }
            }
            
            // Open list is updated with new node values. 
            let mut new_node = Node { g: weight, parent: parent.position, ..node };
            new_node.h = helpers::heuristic(new_node, goal_node, &flags.heur);
            new_node.f = new_node.g + new_node.h;
            open_list.insert(new_node);
            open.push(new_node);
        }

        // Safe guards before we check if node is goal node.
        let mut incumbent_data = incumbent.lock().unwrap();
        
        if open.is_empty() || open.peek().unwrap().f >= incumbent_data.cost {
            continue;
        }
        
        // Process node to see if it's goal node.
        let temp_node = open.pop().unwrap();

        open_list.remove(&temp_node);
        closed_list.insert(temp_node);
        
        if temp_node == goal_node && incumbent_data.cost >= temp_node.g {
            incumbent_data.node = temp_node;
            incumbent_data.cost = temp_node.g;
            
            // Defer exiting thread to start of the loop after receiving any final messages.
            exit = true;
        }
        
        // Force unlocking so that other threads can access it.
        drop(incumbent_data);
        
        let adjacent = vec![(-1, 1), (0, 1), (1, 1), (-1, 0), (1, 0), (-1, -1), (0, -1), (1, -1)];
        

        // First check if valid move, then We will offset to n', and pass off three-tuple
        // to random thread's buffer list.
        for (x, y) in adjacent {
            // Safe guard before we test a movement
            if helpers::is_valid_neighbor(&graph, &temp_node, x, y) {
                // n' is created, now let's put it in a random buffered list.
                let (x_coordinate, y_coordinate) = (temp_node.position.x + x, temp_node.position.y + y);
                let n_prime = Node::new(x_coordinate, y_coordinate, 0, temp_node.g + 1, 0, temp_node.position);
                
                loop {
                    let i = helpers::compute_recipient(&n_prime, &tried, NUM_THREADS as u64); // calculate hash of node to send to a thread.
                    
                    if i == -1 {
                        buffer.push(Buffer(n_prime, n_prime.g, temp_node));
                        break;
                    }

                    match tx[i as usize].send(Buffer(n_prime, n_prime.g, temp_node)) {
                        Ok(_) => {
                            sent_messages.fetch_add(1, Ordering::SeqCst);
                            break;
                        },
                        Err(_) => {
                            tried.insert(i as i32);
                            
                            if tried.len() < NUM_THREADS {
                                continue;
                            }
                            else {
                                break;
                            }
                        },
                    }
                }
            }
        }
    }
}