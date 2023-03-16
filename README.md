# My Rusty Journey 🦀

This Repository contains my experience with topics I've dug deeper into while learning Rust, just because I find them interesting.

Some of these topics would be (for Instance) iterators and constants.

The repository contains multiple crates for each topic, while indiscernible topics have been put together as binaries in the main src directory.

## Folder structure 📁

```txt
.
│      
├── benches/
│   └── soritng_benchmark.rs
│
├── builder_pattern/
│   ├── src/
│   │   ├── lib.rs
│   │   └── main.rs
│   └── Cargo.toml
│
├── closures/
│   ├── src/
│   │   ├── bin/
│   │   │   ├── calculator.rs
│   │   │   └── dyn_vs_static_dispatch.rs
│   │   ├── trait_types/
│   │   │   ├── fn_mut_trait.rs
│   │   │   ├── fn_once_trait.rs
│   │   │   └── fn_trait.rs
│   │   ├── main.rs
│   │   └── trait_types.rs
│   └── Cargo.toml
│
├── constant_fun/
│   ├── src/
│   │   └── bin/
│   │       ├── animals.rs
│   │       ├── buf_concat.rs
│   │       ├── customers.rs
│   │       ├── five_numbers.rs
│   │       ├── german_goodbye.rs
│   │       └── last_numbers.rs
│   └── Cargo.toml
│
├── idiomatic_rust/
│   ├── src/
│   │   ├── lib.rs
│   │   └── main.rs
│   └── Cargo.toml
│
├── iterators/
│   ├── src/
│   │   └── bin/
│   │       ├── combinators.rs
│   │       ├── for_vs_iter.rs
│   │       ├── iter_diff.rs
│   │       ├── ordered_arr.rs
│   │       └── self_impl.rs
│   └── Cargo.toml
│
├── s_t_i_l/
│   ├── src/
│   │   └── bin/
│   │       ├── optimized.rs
│   │       └── unoptimized.rs
│   └── Cargo.toml
│
├── src/
│   ├── bin/
│   │    ├── guessing_game.rs
│   │    ├── my_atoi.rs
│   │    ├── pitfall_game.rs
│   │    └── ref.rs
│   ├── lib.rs
│   └── main.rs
├── .gitignore
├── Cargo.lock
├── Cargo.toml
├── README.md
└── rust-toolchain.toml
```
