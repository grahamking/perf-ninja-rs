Port of https://github.com/dendibakh/perf-ninja

Each lab is a cargo project

layout:
	src/lib.rs				# solution.cpp, solution.h, validate.cpp
	benches/lab.rs			# bench.cpp

Validate:
	cargo test				# validation is a unit test in lib.rs

Benchmark:
	cargo criterion --bench lab --no-run						# Build benchmark, but don't run:
	runperf target/release/deps/lab-14a39128970b4538 --bench	# run the benchmark

Investigate (change '-l1' to '-l2'):
	# https://github.com/andikleen/pmu-tools
	runperf ~/src/pmu-tools/toplev --core S0-C0 -l1 -v --no-desc target/release/deps/lab-14a39128970b4538 --bench

## Thanks

Thanks to my employer Dropbox for supporting this project during Hack Week 2022.

## License

Original problems and ideas Copyright © 2021 by Denis Bakhvalov under Creative Commons license (CC BY 4.0).
Rust port Copyright © 2022 by Graham King under Creative Commons license (CC BY 4.0).

