pub type MatrixOfDoubles = Vec<Vec<f64>>;

pub fn init_matrix(matrix: &mut MatrixOfDoubles) {
    let size = matrix.len();

    for i in 0..size {
        for j in 0..size {
            matrix[i][j] = ((i + j) % 1024) as f64
        }
    }
}

pub fn original_solution(matrix_in: &MatrixOfDoubles, matrix_out: &mut MatrixOfDoubles) {
    let size = matrix_in.len();

    for i in 0..size {
        for j in 0..size {
            matrix_out[i][j] = matrix_in[j][i];
        }
    }
}

pub fn solution(matrix_in: &MatrixOfDoubles, matrix_out: &mut MatrixOfDoubles) {
    const TILE_SIZE: usize = 16;

    let size = matrix_in.len();
    for ii in (0..size).step_by(TILE_SIZE) {
        for jj in (0..size).step_by(TILE_SIZE) {
            for i in ii..std::cmp::min(ii + TILE_SIZE, size) {
                for j in jj..std::cmp::min(jj + TILE_SIZE, size) {
                    matrix_out[i][j] = matrix_in[j][i];
                }
            }
        }
    }
}
