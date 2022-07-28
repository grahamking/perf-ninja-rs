use loop_tiling::{init_matrix, original_solution, solution, MatrixOfDoubles};

fn main() -> Result<(), &'static str> {
    const N: usize = 2001;

    let mut matrix_in = vec![vec![0_f64; N]; N];
    let mut matrix_out = vec![vec![0_f64; N]; N];
    let mut out_golden = vec![vec![0_f64; N]; N];

    init_matrix(&mut matrix_in);

    original_solution(&matrix_in, &mut out_golden);
    solution(&matrix_in, &mut matrix_out);

    if !matricies_equal(&matrix_out, &out_golden) {
        Err("Validation Failed!")
    } else {
        Ok(())
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
