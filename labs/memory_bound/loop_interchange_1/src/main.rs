#![feature(bench_black_box)]

use loop_interchange_1::{init, power, zero, N};

fn main() {
    let mut matrix_a = vec![vec![0.0f32; N]; N];
    init(&mut matrix_a);
    let mut matrix_b = vec![vec![0.0f32; N]; N];
    zero(&mut matrix_b);

    for _ in 0..2 {
        matrix_b = power(&matrix_a, 2021);
        std::hint::black_box(&mut matrix_b);
    }
}

#[cfg(test)]
mod tests {
    use loop_interchange_1::{identity, init, multiply, power, zero, Matrix, N};

    #[test]
    fn validate() {
        const K: i32 = 15;
        const K1: i32 = 5;

        let mut a = vec![vec![0.0f32; N]; N];
        let mut b = vec![vec![0.0f32; N]; N];
        let mut c = vec![vec![0.0f32; N]; N];
        let mut d = vec![vec![0.0f32; N]; N];

        init(&mut a);
        zero(&mut b);
        identity(&mut c);
        identity(&mut d);
        {
            multiply(&mut b, &a, &d);
            assert!(equals(&b, &a));
        }
        {
            multiply(&mut b, &a, &a);
            c = power(&a, 2);
            assert!(equals(&b, &c));
        }
        b = power(&a, K);
        c = power(&a, K1);
        d = power(&a, K - K1);
        multiply(&mut a, &c, &d);
        assert!(equals(&a, &b));
    }

    fn equals(a: &Matrix, b: &Matrix) -> bool {
        const MAX_ERRORS: i32 = 10;
        let mut epsilon: f32 = f32::EPSILON;
        epsilon = epsilon.sqrt();

        let mut errors = 0i32;
        for i in 0..N {
            for j in 0..N {
                let va = a[i][j];
                let vb = b[i][j];
                let error = (va - vb).abs();
                if error >= epsilon {
                    eprintln!("Result[{i}, {j}] = {va}. Expected[{i}, {j}] = {vb}");
                    errors += 1;
                    if errors >= MAX_ERRORS {
                        return false;
                    }
                }
            }
        }

        0 == errors
    }
}
