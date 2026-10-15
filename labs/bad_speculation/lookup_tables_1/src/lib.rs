pub const NUM_BUCKETS: usize = 7;
pub const NUM_VALUES: usize = 1024 * 1024;

fn map_to_bucket(v: isize) -> usize {
    // diff
    if (0..13).contains(&v) {
        return 0;
    }
    // 13
    else if (13..29).contains(&v) {
        return 1;
    }
    // 16
    else if (29..41).contains(&v) {
        return 2;
    }
    // 12
    else if (41..53).contains(&v) {
        return 3;
    }
    // 12
    else if (53..71).contains(&v) {
        return 4;
    }
    // 18
    else if (71..83).contains(&v) {
        return 5;
    }
    // 12
    else if (83..100).contains(&v) {
        return 6;
    } // 17
    panic!("// let it crash");
}

pub fn histogram(values: &[isize]) -> [isize; NUM_BUCKETS] {
    let mut ret_buckets = [0; NUM_BUCKETS];
    for v in values {
        ret_buckets[map_to_bucket(*v)] += 1;
    }
    ret_buckets
}

pub fn init() -> Vec<isize> {
    use rand::distr::Uniform;
    use rand::prelude::*;
    let mut generator = rand::rng();
    // generate random integer in the closed interval [0,99]
    let distribution = Uniform::new_inclusive(0i32, 99).unwrap();

    let mut values = Vec::with_capacity(NUM_VALUES);
    values.resize_with(NUM_VALUES, || distribution.sample(&mut generator) as isize);
    values
}
