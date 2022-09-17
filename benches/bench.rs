use criterion::{black_box, criterion_group, criterion_main, Criterion};
// Import the Universe from lib.rs:
use wasm_game_of_life::universe::{Universe, UniverseOption};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

/// Benchmarks the `tick` method of the `Universe` struct.
///
/// It creates a new universe with the `TwoSeven` option and
/// runs the `tick` method 100 times on it.
fn universe_ticks() {
    let mut universe = Universe::new(UniverseOption::TwoSeven);

    // Tick the universe 100 times:
    for _ in 0..100 {
        universe.tick();
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("tick 100", |b| b.iter(|| universe_ticks()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
