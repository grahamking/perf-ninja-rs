use crate::{MAX_RANDOM, MIN_RANDOM, N, S};

pub fn create_entry(first_value: i32, second_value: i32) -> S {
    S {
        i: first_value,
        l: (first_value * second_value) as i64,
        s: second_value as i16,
        d: first_value as f64 / MAX_RANDOM as f64,
        b: first_value < second_value,
    }
}

// The C++ version uses `std::array`. If I use `[S;N]` I get a stack overflow
// in debug mode (not in release mode).
// Strange because Linux has 8 MB stacks, size_of S is 40 bytes * N ~= 400 KB.
// To be investigated.
pub fn init() -> Vec<S> {
    // [S; N] {
    use rand::distributions::Uniform;
    use rand::prelude::*;
    let mut generator = thread_rng();
    let distribution = Uniform::from(MIN_RANDOM..MAX_RANDOM - 1);

    let mut v = Vec::with_capacity(N);
    //std::array::from_fn(|_| {
    v.resize_with(N, || {
        let random_int1 = distribution.sample(&mut generator);
        let random_int2 = distribution.sample(&mut generator);
        create_entry(random_int1, random_int2)
    });
    v
}
