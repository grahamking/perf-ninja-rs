# Rust labs for Performance Ninja Class

Rust port of the exercises in https://github.com/dendibakh/perf-ninja

You will need to watch the videos at the parent project, that's the course. To do the course in Rust, use this code instead of the parent C++ code.

I recommend reading Denis' free ebook [Performance Analysis and Tuning on Modern CPUs](https://book.easyperf.net/perf_book) as you do the course. Things can get a little confusing otherwise, and the book all by itself is excellent; real practical performance tuning advice from an expert.

## Lab assignments ported so far

* [Core Bound](labs/core_bound):
  * [Vectorization 1](labs/core_bound/vectorization_1)
  * [Vectorization 2](labs/core_bound/vectorization_2)
  * [Function Inlining](labs/core_bound/function_inlining_1)
  * [Dependency Chains 1](labs/core_bound/dep_chains_1)
  * [Compiler Intrinsics 1](labs/core_bound/compiler_intrinsics_1)
  * [Compiler Intrinsics 2](labs/core_bound/compiler_intrinsics_2)
* Misc:
  * [Warmup](labs/misc/warmup)

Note that Rust solves some of these labs all by itself (e.g. `function_inlining`). You might not hit the same performance problems described in the course videos, or you might only hit them in debug mode. Hopefully you'll learn something in finding out if that's the case or not.

## Setup

You need:
 - [Rust](https://www.rust-lang.org/tools/install), obviously.
 - The videos from the parent project: https://github.com/dendibakh/perf-ninja
 - [runperf](https://gist.github.com/grahamking/9c8c91b871843a9a6ce2bec428b8f48d) script to be able to run accurate benchmarks on Linux.
 - [pmu-tools](https://github.com/andikleen/pmu-tools) to do the investigation.

## Layout

Each lab is a cargo project. In brackets are the mappings to the C++ version.

 - `src/lib.rs`: The code you need to optimize (solution.cpp, solution.h, init.cpp)
 - `src/main.rs`: A unit test (validate.cpp) to check your code still works, and a simple `main` function for analysis.
 - `benches/lab.rs`: The benchmark (bench.cpp). You need to make this faster.

You will only need to touch the code in `lib.rs`. The unit test, the benchmark and main all call that code. The benchmark uses [criterion](https://docs.rs/criterion/latest/criterion/) to produce accurate numbers. The downside is that if you analyse that benchmark, you're also analysing criterion. Hence we provide a simple `main` to analyse instead.

## Work loop

1. Improve the code in `lib.rs`.
1. Check it's still correct: `cargo test`.
1. Run the benchmark to see how you're doing: `runperf ~/.cargo/bin/cargo criterion bench --lab` (runperf loses $PATH, hence cargo full path).
1. Build main: `cargo build --release`
1. Analyse it to find bottlenecks - the videos often walk through this part, then go back to step 1. e.g:
   - `runperf perf stat ./target/release/vectorization_1`
   - `runperf perf record ./target/release/vectorization_1` then `perf report -Mintel`.
   - `runperf ~/src/pmu-tools/toplev --core S0-C0 -l1 -v --no-desc target/release/vectorization_1` (then try with `-l2` instead of `-l1`)

## Misc / Tips

Optimize Rust for your CPU, and include frame pointers: `-Ctarget-cpu=native -Cforce-frame-pointers=yes`.

Have `perf report` display the call graph: `perf record --call-graph fp <prog>`. You need to build with `force-frame-pointers` (above in RUSTFLAGS).

Show assembly: `objdump -Mintel -d target/release/vectorization_1 | rustfilt`. (rustfilt de-mangles Rust symbols: `cargo install rustfilt`).

By default `perf record` uses the `cycles` events (number of CPU cycles). If you want to dig into a specific event provide that directly to perf:
 - Branch misses (bad speculation): `runperf perf record --call-graph fp --event=branch-misses:P <prog>`
 - Main memory load (backend bound): `--event=cycle_activity.stalls_l3_miss:P` (An L3 cache miss means we have to go to main memory)

The `:P` denotes a [Precise Event](https://www.intel.com/content/www/us/en/develop/documentation/vtune-help/top/analyze-performance/custom-analysis/custom-analysis-options/hardware-event-list/precise-events.html).

## Thanks

Thanks to my employer Dropbox for supporting this project during Hack Week 2022.

If this course is useful to you please consider supporting the parent project's Patreon or GitHub Sponsors.

## License

Original problems and ideas Copyright © 2021 by Denis Bakhvalov under Creative Commons license (CC BY 4.0).
Rust port Copyright © 2022 by Graham King under Creative Commons license (CC BY 4.0).

