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
    EuclideanDist, Expensive, NonAdmissible
}

const USING_HEUR: HeurType = HeurType::Expensive;

fn distance(node: Node, end: Node) -> i128
{
	(((end.position.x - node.position.x).pow(2) + (end.position.y - node.position.y).pow(2)) 
	        as f32).sqrt() as i128
}

fn expensive(node: Node, end: Node) -> i128
{

    // need to import via cargo
    // let mut rng = rand::thread_rng();

    // let time = rng.gen_range(100..1000);

    // let rand_millis = time::Duration::from_millis(time);
    // thread::sleep(rand_millis);

	(((end.position.x - node.position.x).pow(2) + (end.position.y - node.position.y).pow(2)) 
	as f32).sqrt() as i128
}

fn heuristic(node: Node, end: Node) -> i128
{
    if USING_HEUR == HeurType::EuclideanDist
    {
        return distance(node, end);
    }
    else if USING_HEUR == HeurType::Expensive
    {

    }
    else if USING_HEUR == HeurType::Expensive
    {

    }
    
    // Won't occur
    0
}


fn setup()
{

    let mut threads = Vec::with_capacity(NUMTHREADS);

    let mut start = Node::default();
    let end = Node::new(1, 1, 0, 0, 0, Point::default());
    start.h = distance(start, end);


}