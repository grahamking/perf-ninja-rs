#![feature(bench_black_box)]

use warmup::solution;

fn main() {
    const N: usize = 1000;
    let mut arr = [0i32; N];
    for i in 0..N {
        arr[i] = i as i32 + 1i32;
    }

    for _ in 0..300_000 {
        let result = solution(&arr, N);
        std::hint::black_box(result);
    }
}

#[cfg(test)]
mod tests {

    use warmup::solution;

    #[test]
    fn validate() {
        const N: usize = 1000;
        let mut arr = [0i32; N];
        for i in 0..N {
            arr[i] = i as i32 + 1i32;
        }

        let result = solution(&arr, N);
        assert_eq!(result as usize, (N * (N + 1)) / 2, "Validation Failed");
    }
}
