# AOC22 implementation using `no_std`

This is an experiment to try and implement as many of the AOC puzzles with limited RAM and no heap allocation.

As a learning exercise, I'm using `nom` (without `alloc`) for most of the parsing and `heapless` for some useful data structures.

If you have QEMU installed, you can just `cargo run --release --bin day01` to run it on a simulated `lm3s6965evb`. It has 64KB of RAM and 256KB of Flash.