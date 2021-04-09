#[macro_use]
extern crate clap;
mod a_star;
use a_star::{
    utils::structs::{HeurType, Flags},
    hda,
    dpa,
    kpbfs,
    utils::helpers
};

fn validate_heuristic(heur: String) -> Result<(), String> {
    match heur.as_str() {
        "euclidean" => Ok(()),
        "manhattan" => Ok(()),
        "expensive" => Ok(()),
        "nonadmissible" => Ok(()),
        "expnon" => Ok(()),
        _ => Err(String::from("Please input a valid heuristic option [euclidean, manhattan, expensive, nonadmissible, expnon]")),
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

    let heur_type = match config.value_of("HUERISTIC").unwrap_or("euclidean") {
        "euclidean" => HeurType::EuclideanDist,
        "manhattan" => HeurType::ManhattanDist,
        "expensive" => HeurType::Expensive,
        "nonadmissible" => HeurType::NonAdmissible,
        "expnon" => HeurType::ExpensiveNonAdmissible,
        _ => HeurType::EuclideanDist,
    };

    let threads = config.value_of("NUM_THREADS").unwrap_or("4").parse().unwrap_or(4);
    let (graph, start, end) = helpers::parse_graph(config.value_of("GRAPH"));
    let flags = Flags { graph, heur: heur_type, threads: threads };
    let algo = config.value_of("ALGO").unwrap_or("hda");

    match algo {
        "hda" => hda::setup(start, end, flags),
        "dpa" => dpa::setup(start, end, flags),
        "kpbfs" => kpbfs::setup(start, end, flags),
        _ => hda::setup(start, end, flags),
    }
}