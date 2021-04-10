use criterion::{black_box, criterion_group, criterion_main, Criterion};
mod a_star;
use a_star::{
    utils::structs::{HeurType, Flags},
    hda,
    dpa,
    kpbfs,
    utils::helpers::{parse_graph}
};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

fn string_from_heur(heur: HeurType) -> String {
    let heur_str = match heur {
        HeurType::EuclideanDist => "euclidean",
        HeurType::ManhattanDist => "manhattan",
        HeurType::Expensive => "expensive",
        HeurType::NonAdmissible => "nonadmissible",
        HeurType::ExpensiveNonAdmissible => "expnon",
    };

    return heur_str.to_string();
}

fn criterion_benchmark(c: &mut Criterion) {
    // each algo, thread count, heuristic types
    // algo_#t_heurtype
    // Threads: 1, 2, 4, 8, 16, 32
    let thread_cnts: [usize; 6] = [1, 2, 4, 8, 16, 32];
    // heurtype: all
    let heurs: [HeurType; 5] = [
        HeurType::ManhattanDist,
        HeurType::EuclideanDist,
        HeurType::Expensive,
        HeurType::NonAdmissible,
        HeurType::ExpensiveNonAdmissible
    ];

    let algo_type: [String; 3] = ["kpbfs".to_string(), "dpa".to_string(), "hda".to_string()];    

    let input = "large1.in";

    // Reduce the sample size for PA* algos.
    let mut group = c.benchmark_group("pa");

    group.sample_size(10);

    for cnt in thread_cnts.iter() {
        for heur_type in heurs.iter() {
            for algo in algo_type.iter() {
    
                let format = format!("{}_{}t_{}", *algo, (*cnt).to_string(), string_from_heur(*heur_type));

                // Graph nor flags is copyable
                let (_, start, end) = parse_graph(Some(input));

                match algo.as_ref() {
                    "hda" => {
                        group.bench_function(&format, |b| b.iter(|| hda::setup(start, end,
                            Flags { graph: parse_graph(Some(input)).0, heur: *heur_type, threads: *cnt })))
                    },
                    "dpa" => {
                        group.bench_function(&format, |b| b.iter(|| dpa::setup(start, end,
                            Flags { graph: parse_graph(Some(input)).0, heur: *heur_type, threads: *cnt })))
                    },
                    "kpbfs" => {
                        group.bench_function(&format, |b| b.iter(|| kpbfs::setup(start, end,
                            Flags { graph: parse_graph(Some(input)).0, heur: *heur_type, threads: *cnt })))
                    },
                    _ => { 
                        group.bench_function(&format, |b| b.iter(|| hda::setup(start, end,
                            Flags { graph: parse_graph(Some(input)).0, heur: *heur_type, threads: *cnt })))
                    },
                };

                


            }
        }
    };

    group.finish();

    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);