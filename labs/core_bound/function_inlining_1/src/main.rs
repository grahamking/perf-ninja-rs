#![feature(bench_black_box)]
use function_inlining_1::{init, solution};

fn main() {
    let arr = init();

    for _ in 0..5_000 {
        let mut copy = arr;
        solution(&mut copy);
        std::hint::black_box(copy);
    }
}

#[cfg(test)]
mod tests {
    use function_inlining_1::{init, solution, N, S};
    use std::cmp::Ordering;

    #[test]
    fn validate() {
        let mut arr = init();

        let mut expected = arr; // copy
        solution(&mut arr);
        original_solution(&mut expected);

        assert_eq!(arr, expected);
    }

    fn original_compare(a: &S, b: &S) -> Ordering {
        if a.key1 < b.key1 {
            return Ordering::Less;
        }

        if a.key1 > b.key1 {
            return Ordering::Greater;
        }

        if a.key2 < b.key2 {
            return Ordering::Less;
        }

        if a.key2 > b.key2 {
            return Ordering::Greater;
        }

        Ordering::Equal
    }

    fn original_solution(arr: &mut [S; N]) {
        arr.sort_unstable_by(original_compare);
    }
}
