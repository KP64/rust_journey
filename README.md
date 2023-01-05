# My Rusty Journey 🦀

This Repository contains my experience with topics I've dug deeper into while learning Rust just because I find them interesting.

Some of these topics would (for example) be iterators and constants.

The repository contains multiple crates for each topic, while indiscernible topics have been put together as binaries in the main src directory.

## Folder structure 📁

```rs
.
│       
├── closures/
│   ├── src/
│   │   ├── trait_types/
│   │   │   ├── fn_mut_trait.rs
│   │   │   ├── fn_once_trait.rs
│   │   │   ├── fn_trait.rs
│   │   │   └── mod.rs
│   │   ├── calculator.rs
│   │   ├── dyn_vs_static_dispatch.rs
│   │   └── main.rs
│   └── Cargo.toml
│
├── constant_fun/
│   ├── src/
│   │   ├── animals.rs
│   │   ├── buf_concat.rs
│   │   ├── customers.rs
│   │   ├── five_numbers.rs
│   │   ├── german_goodbye.rs
│   │   ├── last_numbers.rs
│   │   └── main.rs
│   └── Cargo.toml
│
├── iterators/
│   ├── src/
│   │   ├── mod iter_diff: pub(crate)
│   │   ├── main
│   │   ├── mod ordered_arr: pub(crate)
│   │   └── mod self_impl: pub(crate)
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
│   └── bin/
│       ├── binary_search.rs
│       ├── combinators.rs
│       ├── for_vs_iter.rs
│       ├── guessing_game.rs
│       ├── main.rs
│       ├── pitfall_game.rs
│       └── ref.rs
├── .gitignore
├── Cargo.lock
├── Cargo.toml
└── README.md
```
