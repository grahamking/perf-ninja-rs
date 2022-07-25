#![feature(bench_black_box)]
use vectorization_2::{checksum, init, Blob, N};

fn main() {
    let mut blob: Blob = [0; N];
    init(&mut blob);
    for _ in 0..100_000 {
        let result = checksum(&blob);
        std::hint::black_box(result);
    }
}

#[cfg(test)]
mod tests {
    use std::num::Wrapping;
    use vectorization_2::{checksum, init, Blob, N};

    fn original_checksum(blob: &Blob) -> u16 {
        let mut acc = Wrapping(0);
        for value in blob {
            acc += value;
            acc += (acc.0 < *value) as u16; // add carry
        }
        acc.0
    }

    #[test]
    fn validate() {
        let mut blob: Blob = [0; N];
        init(&mut blob);

        let original_result = original_checksum(&blob);
        let result = checksum(&blob);

        assert_eq!(original_result, result);
    }
}
