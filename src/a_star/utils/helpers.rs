use std::{collections::{HashSet, hash_map::DefaultHasher}, hash::{Hash, Hasher}};
use super::structs::{HeurType, Node};

fn euclidean(node: Node, end: Node) -> i128 {
    (((end.position.x - node.position.x).pow(2) + (end.position.y - node.position.y).pow(2)) as f32)
        .sqrt() as i128
}

fn manhattan(node: Node, end: Node) -> i128 {
    ((node.position.x - end.position.x).abs() + (node.position.y - end.position.y).abs()) as i128
}

pub fn heuristic(node: Node, end: Node, heur: &HeurType) -> i128 {
    match heur {
        HeurType::EuclideanDist => euclidean(node, end),
        HeurType::ManhattanDist => manhattan(node, end)
    }
}

/// Basic bounds checking
pub fn is_valid_neighbor(graph: &Vec<Vec<char>>, node: &Node, x: i32, y: i32) -> bool {
    let (x0, y0) = (node.position.x + x, node.position.y + y);

    x0 >= 0 && y0 >= 0 && x0 < graph.len() as i32 && y0 < graph.len() as i32
        && graph[x0 as usize][y0 as usize] != 'W'
}

/// Calculate index using hashed node for thread to send Buffer() to.
/// Returns -1 if in last thread.
pub fn compute_recipient(node: &Node, setty: &HashSet<i32>, num_threads: u64) -> i32 {
    let mut index;
    let hash = (|| {
        let mut state = DefaultHasher::new();

        (&node).hash(&mut state);
        state.finish()
    })();

    for i in 0..num_threads {
        index = (hash + i) % num_threads;
        // Makes sure we don't index the same thread's channel or a dead channel
        if setty.contains(&(index as i32)) {
            continue;
        }
        else {
            return index as i32;
        }
    }
    
    return -1;
}