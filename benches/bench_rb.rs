use std::fmt::format;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tree::tree_type::RedBlackTree as RBT;


fn rb_insert_worst_case(tree: &mut RBT<u128>, tree_size: &u128) {
    for key in 0..tree_size.clone() {
        tree.insert(&key);
    }
}

fn rb_search_worst_case(tree: &RBT<u128>, tree_size: &u128) {
    for key in 0..tree_size/10 {
        let _ = tree.search(&key);
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    for tree_size in [10_000, 40_000, 70_000, 100_000, 130_000] {
        let mut tree: RBT<u128> = RBT::new();
        c.bench_function(
            format!("rb_insert_{:?}", tree_size).as_str(),
            |b| b.iter(|| rb_insert_worst_case(
                black_box(&mut tree), 
                black_box(&tree_size)
            ))
        );
        c.bench_function(
            format!("rb_search_{:?}", tree_size/10).as_str(),
            |b| b.iter(|| rb_search_worst_case(
                black_box(&tree), 
                black_box(&tree_size)
            ))
        );
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);