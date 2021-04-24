use std::{collections::{HashSet, hash_map::DefaultHasher}, hash::{Hash, Hasher}, usize};
use super::structs::{HeurType, Node, Point};

use std::fs::File;
use std::io::{BufRead, BufReader};

use std::{thread, time};
use rand::Rng;

fn euclidean(node: Node, end: Node) -> i128 {
    (((end.position.x - node.position.x).pow(2) + (end.position.y - node.position.y).pow(2)) as f32)
        .sqrt() as i128
}

fn manhattan(node: Node, end: Node) -> i128 {
    ((node.position.x - end.position.x).abs() + (node.position.y - end.position.y).abs()) as i128
}

fn random_wait() {
    //need to import via cargo
    //let mut rng = rand::thread_rng();

    //let time = rng.gen_range(1..5);
    let rand_millis = time::Duration::from_millis(1);
    thread::sleep(rand_millis);
}

fn expensive(node: Node, end: Node) -> i128 {
    random_wait();

    euclidean(node, end)
}

fn non_admissible(node: Node, end: Node, expensive: bool) -> i128 {
    if expensive {
        random_wait();
    }

    let mut rng = rand::thread_rng();

    let percent = rng.gen_range(1.0..100.0);

    let dist = euclidean(node, end);

    let result = (dist as f64) + (percent / 100.0) * (dist as f64);

    result as i128
}

pub fn heuristic(node: Node, end: Node, heur: &HeurType) -> i128 {
    match heur {
        HeurType::EuclideanDist => euclidean(node, end),
        HeurType::ManhattanDist => manhattan(node, end),
        HeurType::Expensive => expensive(node, end),
        HeurType::NonAdmissible => non_admissible(node, end, false),
        HeurType::ExpensiveNonAdmissible => non_admissible(node, end, true)
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
pub fn compute_recipient(node: &Node, setty: &HashSet<i32>, num_threads: u64, thread_num: usize) -> i32 {
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
    
    return thread_num as i32;
}

pub fn parse_graph(graph_file: Option<&str>) -> (Vec<Vec<char>>, Point, Point) {
    let file = File::open("data/".to_owned() + graph_file.unwrap_or("medium1.in"))
        .expect("Could not open file");
    let mut fp = (BufReader::new(file)).lines();
    let size = fp.next().unwrap().unwrap().parse::<usize>().unwrap();
    let mut graph: Vec<Vec<char>> = Vec::with_capacity(size);

    for line in fp {
        graph.push(line.unwrap().chars().collect());
    }

    let mut start_point = Point::default();
    let mut end_point = Point::default();

    for i in 0..graph.len() {
        for j in 0..graph.len() {
            if graph[i][j] == 'S' {
                start_point.x = i as i32;
                start_point.y = j as i32;
            }
            if graph[i][j] == 'E' {
                end_point.x = i as i32;
                end_point.y = j as i32;
            }
        }
    }

    (graph, start_point, end_point)
}