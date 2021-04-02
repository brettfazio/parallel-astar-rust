#[macro_use]
extern crate clap;
use std::fs::File;
use std::io::{BufRead, BufReader};
mod a_star;
use a_star::{
    utils::structs::{Point, HeurType, Flags},
    hda,
    dpa
};

fn parse_graph(graph_file: Option<&str>) -> (Vec<Vec<char>>, Point, Point) {
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

fn validate_heuristic(heur: String) -> Result<(), String> {
    match heur.as_str() {
        "euclidean" => Ok(()),
        "manhattan" => Ok(()),
        _ => Err(String::from("Please input a valid heuristic option [euclidean, manhattan]")),
    }
}

fn validate_algo(algo: String) -> Result<(), String> {
    match algo.as_str() {
        "hda" => Ok(()),
        "dpa" => Ok(()),
        "kpbfs" => Ok(()),
        _ => Err(String::from("Please input a valid implementation option [hda, dpa, kpbfs]")),
    }
}

fn main() {
    // Will need to add a sequential {breadth/best}FS as well as additional heuristic types.
    // and pass flags to kpbfs, dpa, and any other impls
    let config = clap_app!(a_star =>
        (@arg GRAPH: -g --graph +takes_value "Graph to use for algorithm implementation")
        (@arg HEURISTIC: -h --heur +takes_value { validate_heuristic } "Heuristic type to use")
        (@arg NUM_THREADS: -n --num_threads +takes_value "Number of threads to use")
        (@arg ALGO: -a --algo +takes_value { validate_algo } "Underlying algorithm to use" )
        (@arg debug: -d "Set debugging flag")
    ).get_matches();

    // Example cargo run -- --graph large2.in --algo hda

    let (graph, start, end) = parse_graph(config.value_of("GRAPH"));
    let mut flags = Flags { graph, heur: HeurType::ManhattanDist };
    let algo = config.value_of("ALGO").unwrap_or("hda");

    match algo {
        "hda" => hda::setup(start, end, flags),
        "dpa" => dpa::setup(start, end, flags),
        _ => hda::setup(start, end, flags),
    }
}