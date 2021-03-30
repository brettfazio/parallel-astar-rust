#[macro_use]
extern crate clap;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod hda;
use hda::structs::{HeurType, Flags};

fn validate_heuristic(heur: String) -> Result<(), String> {
    match heur.as_str() {
        "euclidean" => Ok(()),
        "manhattan" => Ok(()),
        _ => Err(String::from("Please input a valid heuristic option [euclidean, manhattan]")),
    }   
}

fn main() {
    let config = clap_app!(hda =>
        (version: "Amazing version")
        (@arg GRAPH: -g --graph +takes_value "Graph to use for algorithm implementation")
        (@arg HEURISTIC: -h --heur +takes_value { validate_heuristic } "Heuristic type to use")
        (@arg NUM_THREADS: -n --num_threads +takes_value "Number of threads to use")
        (@arg debug: -d "Set debugging flag")
    ).get_matches();

    let file = File::open("../data/medium/".to_owned() + config.value_of("GRAPH").unwrap_or("medium1.in")).expect("Could not open file");
    let mut fp = (BufReader::new(file)).lines();
    let size = fp.next().unwrap().unwrap().parse::<usize>().unwrap();
    let mut graph: Vec<Vec<char>> = Vec::with_capacity(size);

    for line in fp {
        graph.push(line.unwrap().chars().collect());
    }

  let mut flags = Flags { graph, heur: HeurType::ManhattanDist };
    

    hda::setup(flags);
}