#![feature(bench_black_box)]

use false_sharing_1::solution;

fn main() {
    const SIZE: usize = 1024 * 1024;

    let mut data = Vec::with_capacity(SIZE);
    let mut i: i32 = -1;
    data.resize_with(SIZE, || {
        i += 1;
        i as u32
    });

    let max_threads = std::thread::available_parallelism().unwrap().get();
    for _ in 0..100 {
        for thread_count in 1..=max_threads {
            let result = solution(&data, thread_count);
            std::hint::black_box(result);
        }
    }
}

#[cfg(test)]
mod tests {

    use false_sharing_1::solution;

    #[test]
    fn validate() {
        const SIZE: usize = 16 * 1024 * 1024;

        let mut data = Vec::with_capacity(SIZE);
        let mut i: i32 = -1;
        data.resize_with(SIZE, || {
            i += 1;
            i as u32
        });

        let original_result = original_solution(&data);

        // Use thread count from 1 to <number of HW threads>
        let max_threads = std::thread::available_parallelism().unwrap().get();
        let mut threads = Vec::with_capacity(max_threads);
        let mut i = 0;
        threads.resize_with(max_threads, || {
            i += 1;
            i
        });

        for thread_count in threads {
            let result = solution(&data, thread_count);
            if original_result != result {
                eprintln!(
                    "Validation Failed for {thread_count} thread(s).
        Original result = {original_result}; Modified version returned = {result}"
                );
                panic!("Test failed");
            }
        }

        fn original_solution(data: &[u32]) -> i32 {
            let mut value: i32 = 0;

            for i in 0..data.len() {
                let mut item = data[i];
                item += 1000;
                item ^= 0xADEDAE;
                item |= item >> 24;

                value += (item % 13) as i32;
            }

            value
        }
    }
}
