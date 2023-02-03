use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use rust_journey::{
    searching::{binary_search, const_fun::const_binary_search},
    sorting::{
        bubble_sort,
        const_fun::{const_bubble_sort, const_selection_sort},
        selection_sort,
    },
};

const KEY: i32 = 10;
fn sort_arr_benchmark(c: &mut Criterion) {
    let mut arr = black_box([
        6, 2, 4, 1, 9, -2, 5, 10, 100, 20, -124, 208, 29, 254, 32, 32, 1, 11, 20,
    ]);

    c.bench_function("sel sort", |b| b.iter(|| bubble_sort(&mut arr)))
        .bench_function("bub sort", |b| b.iter(|| selection_sort(&mut arr)))
        .bench_function("bin-bub", |b| {
            b.iter(|| {
                bubble_sort(&mut arr);
                binary_search(arr, KEY);
            })
        })
        .bench_function("bin-sel", |b| {
            b.iter(|| {
                selection_sort(&mut arr);
                binary_search(arr, KEY);
            })
        });
}

fn const_sort_arr_benchmark(c: &mut Criterion) {
    let mut arr = black_box([
        6, 2, 4, 1, 9, -2, 5, 10, 100, 20, -124, 208, 29, 254, 32, 32, 1, 11, 20,
    ]);

    c.bench_function("const sel sort", |b| b.iter(|| const_bubble_sort(&mut arr)))
        .bench_function("const bub sort", |b| {
            b.iter(|| const_selection_sort(&mut arr))
        })
        .bench_function("const bin-bub", |b| {
            b.iter(|| {
                const_bubble_sort(&mut arr);
                const_binary_search(&arr, &KEY);
            })
        })
        .bench_function("const bin-sel", |b| {
            b.iter(|| {
                const_selection_sort(&mut arr);
                const_binary_search(&arr, &KEY);
            })
        });
}
criterion_group!(benches, sort_arr_benchmark, const_sort_arr_benchmark);
criterion_main!(benches);
