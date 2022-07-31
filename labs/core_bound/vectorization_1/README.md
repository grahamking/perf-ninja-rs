Use debug mode (not `--release`) for this one. Under Clang 14 in release mode LLVM already vectorizes `compute_alignment`.

Use `cargo build`, `cargo run` and `cargo criterion --bench lab --debug`.

