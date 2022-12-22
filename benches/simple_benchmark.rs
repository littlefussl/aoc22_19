use criterion::{black_box, criterion_group, criterion_main, Criterion};

use day19::blueprint::Blueprint;
use day19::max_search::{traverse_depth_first, MetaState};

static EXAMPLE1: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.";

fn example1(input: &str) -> u8 {
    let blueprint = Blueprint::from(input);
    traverse_depth_first(
        Default::default(),
        0,
        &blueprint,
        &mut MetaState::with_max_steps(10),
    )
    .unwrap_or_default()
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("example1", |b| b.iter(|| example1(black_box(EXAMPLE1))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
