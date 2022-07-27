#![feature(new_uninit)]

// Assume this constant never changes
pub const N: usize = 400;

// Square matrix 400 x 400
pub type Matrix = [[f32; N]; N];

// Make zero matrix
pub fn zero(result: &mut Matrix) {
    for i in 0..N {
        for j in 0..N {
            result[i][j] = 0.0;
        }
    }
}

// Make identity matrix
pub fn identity(result: &mut Matrix) {
    for i in 0..N {
        for j in 0..N {
            result[i][j] = 0.0;
        }
        result[i][i] = 1.0;
    }
}

// Multiply two square matrices
pub fn multiply(result: &mut Matrix, a: &Matrix, b: &Matrix) {
    zero(result);

    for i in 0..N {
        for j in 0..N {
            for k in 0..N {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }
}

// Compute integer power of a given square matrix
pub fn power(input: &Matrix, k: i32) -> Box<Matrix> {
    // Temporary products
    let mut product_current = Box::new([[0.0f32; N]; N]);
    let mut product_next = Box::new([[0.0f32; N]; N]);

    // Temporary elements = a^(2^integer)
    let mut element_current = Box::new([[0.0f32; N]; N]);
    let mut element_next = Box::new([[0.0f32; N]; N]);

    // Initial values
    identity(&mut product_current);
    *element_current = *input; // copy

    // Use binary representation of k to be O(log(k))
    let mut i = k;
    while i > 0 {
        if i % 2 != 0 {
            // Multiply the product by element
            multiply(&mut product_next, &product_current, &element_current);
            std::mem::swap(&mut product_next, &mut product_current);

            // Exit early to skip next squaring
            if i == 1 {
                break;
            }
        }

        // Square an element
        multiply(&mut element_next, &element_current, &element_current);
        std::mem::swap(&mut element_next, &mut element_current);

        i /= 2;
    }

    product_current
}

pub fn init(matrix: &mut Matrix) {
    use rand::distributions::Uniform;
    use rand::prelude::*;

    let mut generator = thread_rng();
    let distribution = Uniform::from(-0.95f32..0.95f32);

    for i in 0..N {
        let mut sum = 0f32;
        for j in 0..N {
            let value = distribution.sample(&mut generator);
            sum += value * value;
            matrix[i][j] = value;
        }

        // Normalize rows
        if sum >= f32::MIN {
            let scale = 1.0f32 / sum.sqrt();
            for j in 0..N {
                matrix[i][j] *= scale;
            }
        }
    }
}
