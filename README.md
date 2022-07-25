Port of https://github.com/dendibakh/perf-ninja

You will need to watch the videos at the parent project, that's the course. To do the course in Rust, use this code instead of the parent C++ code.

## Layout

Each lab is a cargo project.

 - `src/lib.rs`: solution.cpp, solution.h, validate.cpp
 - `benches/lab.rs`: bench.cpp

## Validate

Validation checks that your code is correct. In the parent project this is in separate file `validate.cpp`. Here it's a unit test in `lib.rs`, so just run `cargo test`.

## Benchmark:

To run repeatable benchmarks on Linux you need to adjust some settings. Use this [runperf](https://gist.github.com/grahamking/9c8c91b871843a9a6ce2bec428b8f48d) script.

We use [criterion](https://docs.rs/criterion/latest/criterion/) to run the benchmark.

Build the benchmark: `cargo criterion --bench lab --no-run`.

Run it: `runperf target/release/deps/lab-14a39128970b4538 --bench`. **The time displayed here is what you're trying to improve**.

## Investigate (change '-l1' to '-l2'):

Get the tools: https://github.com/andikleen/pmu-tools

`runperf ~/src/pmu-tools/toplev --core S0-C0 -l1 -v --no-desc target/release/deps/lab-14a39128970b4538 --bench`

## Thanks

Thanks to my employer Dropbox for supporting this project during Hack Week 2022.

## License

Original problems and ideas Copyright © 2021 by Denis Bakhvalov under Creative Commons license (CC BY 4.0).
Rust port Copyright © 2022 by Graham King under Creative Commons license (CC BY 4.0).

