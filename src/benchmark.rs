use criterion::{black_box, criterion_group, criterion_main, Criterion};
mod a_star;
use a_star::{
    utils::structs::{Point, HeurType, Flags},
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

    

    for cnt in thread_cnts.iter() {
        for heur_type in heurs.iter() {
            let (graph, start, end) = parse_graph(Some("large1.in"));
            let flags = Flags { graph, heur: *heur_type, threads: *cnt };
        }
    }

    ;

    //c.bench_function("dpa_8t", )
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);