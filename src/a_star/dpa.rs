use self::atomic::{AtomicU64, Ordering};
use ::atomic::Atomic;
use crossbeam::channel::{Sender, Receiver, unbounded};
use std::{
    thread,
    mem::drop,
    collections::{HashSet, BinaryHeap},
    sync::{Arc, atomic}
};
use super::utils::{
    structs::{Incumbent, Node, Point, Buffer, Flags},
    dynamic_barrier::DynamicHurdle,
    helpers
};

pub fn setup(start_point: Point, end_point: Point, flags: Flags) {
    let Flags { heur, graph, threads: thread_cnt } = flags;
    let mut threads = Vec::with_capacity(thread_cnt);
    let mut receivers: Vec<Receiver<Buffer>> = Vec::with_capacity(thread_cnt);
    let mut transmitters: Vec<Sender<Buffer>> = Vec::with_capacity(thread_cnt);
    let mut barrier = DynamicHurdle::new(thread_cnt);
    let sent_messages = Arc::new(AtomicU64::new(0));
    let received_messages = Arc::new(AtomicU64::new(0));

    // Declares channels
    for _ in 0..thread_cnt {
        let (tx, rx) = unbounded();
            
        transmitters.push(tx);
        receivers.push(rx);
    }

    let mut start = Node::new(start_point.x, start_point.y, 0, 0, 0, Point::default());
    let end = Node::new(end_point.x, end_point.y, 0, 0, 0, Point::default());
    start.h = helpers::heuristic(start, end, &flags.heur);
    start.f = start.g + start.h;
    let incumbent: Arc<Atomic<Incumbent>> = Arc::new(Atomic::new(Incumbent::new(start, i128::MAX)));

    // Here, we would give each thread a different node to start on.
    // Those threads would run A* on each of their respective start nodes.
    for i in 0..thread_cnt {
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

    let final_incumbent = incumbent.load(Ordering::SeqCst);

    println!("All threads found goal node {},{}. Cost of {}",
            final_incumbent.node.position.x, 
            final_incumbent.node.position.y,
            final_incumbent.cost);
}

// A* implementation
fn search(start: Node, thread_num: usize, rx: Receiver<Buffer>, tx: Vec<Sender<Buffer>>,
          mut barrier: DynamicHurdle, goal_node: Node, incumbent: Arc<Atomic<Incumbent>>,
          _graph: Vec<Vec<char>>, sent_messages: Arc<AtomicU64>, received_messages: Arc<AtomicU64>,
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
        
        // Barrier is implicitly dropped, no need to drop it.
        if exit {
            drop(rx);
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

        if open.is_empty() || open.peek().unwrap().f >= incumbent.load(Ordering::SeqCst).cost {
            continue;
        }
        
        // Process node to see if it's goal node.
        let temp_node = open.pop().unwrap();

        open_list.remove(&temp_node);
        closed_list.insert(temp_node);
        
        while temp_node == goal_node && incumbent.load(Ordering::SeqCst).cost > temp_node.g {
            let temp = incumbent.load(Ordering::SeqCst);

            if temp.cost >= temp_node.g {
                let mut new_incumbent = temp;
                new_incumbent.node = temp_node;
                new_incumbent.cost = temp_node.g;
                match incumbent.compare_exchange(temp, new_incumbent, Ordering::SeqCst, Ordering::SeqCst) {
                    Ok(_) => {
                        exit = true;
                    },
                    Err(_) => {
                    }
                }
            }
        }
        
        let adjacent = vec![(0, 1), (-1, 0), (1, 0), (0, -1)];        

        // First check if valid move, then We will offset to n', and pass off three-tuple
        // to random thread's buffer list.
        for (x, y) in adjacent {
            // Safe guard before we test a movement
            if helpers::is_valid_neighbor(&flags.graph, &temp_node, x, y) {
                // n' is created, now let's put it in a random buffered list.
                let (x_coordinate, y_coordinate) = (temp_node.position.x + x, temp_node.position.y + y);
                let n_prime = Node::new(x_coordinate, y_coordinate, 0, temp_node.g + 1, 0, temp_node.position);
                
                loop {
                    let i = helpers::compute_recipient(&n_prime, &tried, flags.threads as u64, thread_num); // calculate hash of node to send to a thread.
                    

                    match tx[i as usize].send(Buffer(n_prime, n_prime.g, temp_node)) {
                        Ok(_) => {
                            sent_messages.fetch_add(1, Ordering::SeqCst);
                            break;
                        },
                        Err(_) => {
                            tried.insert(i as i32);
                            
                            if tried.len() < flags.threads as usize {
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