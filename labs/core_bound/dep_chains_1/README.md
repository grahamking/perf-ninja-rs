Under Clang 14 this is memory bound, not core bound. The Rust version matches the C++ version that way. It's possible this won't teach you what the lab intends. Reduce / remove the dependency chains anyway, and see if the benchmark shows a performance improvement.

