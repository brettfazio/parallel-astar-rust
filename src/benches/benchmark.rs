use criterion::{black_box, criterion_group, criterion_main, Criterion};
// use super::a_star::{
//     utils::structs::{Point, HeurType, Flags},
//     hda,
//     dpa
// };

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
    //c.bench_function("dpa_8t", )
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);