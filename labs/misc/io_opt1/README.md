An included build script generates a necessary module (`src/data_paths.rs`) and some test files. If things aren't working right try `cargo clean` then `cargo build`.

Until you optimize, the "Large file" benchmark might be unpleasantly slow. You can comment it out in `benches/lab.rs` and restore it after optimization.

`MappedFile.hpp` from the C++ original is not yet included, but you should be able to get huge speedups without it.
