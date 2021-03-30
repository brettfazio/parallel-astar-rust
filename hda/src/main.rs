use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod hda;

fn main() {
	let mut args: Vec<String> = env::args().collect();

	if args.len() < 2 {
		args.push("medium1.in".to_string());
	}

	let file = File::open("../data/medium/".to_owned() + &args[1]).expect("Could not open file");
	let mut fp = (BufReader::new(file)).lines();
	let size = fp.next().unwrap().unwrap().parse::<usize>().unwrap();
	let mut graph: Vec<Vec<char>> = Vec::with_capacity(size);

	for line in fp {
		graph.push(line.unwrap().chars().collect());
	}

	hda::setup(graph);
}