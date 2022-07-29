#![feature(bench_black_box)]
use loop_tiling_1::{init_matrix, solution};

fn main() {
    const N: usize = 2001;

    let mut matrix_in = vec![vec![0_f64; N]; N];
    init_matrix(&mut matrix_in);

    let mut matrix_out_solution = vec![vec![0_f64; N]; N];

    for _ in 0..50 {
        solution(&matrix_in, &mut matrix_out_solution);
        std::hint::black_box(&mut matrix_out_solution);
    }
}

#[cfg(test)]
mod tests {

    use loop_tiling_1::{init_matrix, solution, MatrixOfDoubles};

    #[test]
    fn validate() {
        const N: usize = 2001;

        let mut matrix_in = vec![vec![0_f64; N]; N];
        let mut matrix_out = vec![vec![0_f64; N]; N];
        let mut out_golden = vec![vec![0_f64; N]; N];

        init_matrix(&mut matrix_in);

        original_solution(&matrix_in, &mut out_golden);
        solution(&matrix_in, &mut matrix_out);

        assert!(matricies_equal(&matrix_out, &out_golden));
    }

    fn original_solution(matrix_in: &MatrixOfDoubles, matrix_out: &mut MatrixOfDoubles) {
        let size = matrix_in.len();

        for i in 0..size {
            for j in 0..size {
                matrix_out[i][j] = matrix_in[j][i];
            }
        }
    }

    fn matricies_equal(m1: &MatrixOfDoubles, m2: &MatrixOfDoubles) -> bool {
        if m1.len() != m2.len() {
            return false;
        }

        let size = m1.len();
        for i in 0..size {
            for j in 0..size {
                if m1[i][j] != m2[i][j] {
                    return false;
                }
            }
        }

        true
    }
}
