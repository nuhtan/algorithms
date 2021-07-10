/// A basic matrix multiplication function for comparison with Strassen's algorithm.
///
/// TODO This method should have some testing.
pub fn square_matrix_multiply(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = a.len();
    let mut c: Vec<Vec<i32>> = Vec::new();
    for i in 0..n {
        for j in 0..n {
            c[i][j] = 0;
            for k in 0..n {
                c[i][j] = c[i][j] + a[i][k] * b[k][j];
            }
        }
    }
    c
}

/// This is a more efficient modification of standard recursive matrix multiplication.
///
/// # Assumptions:
/// - The matrices [a] and [b] are square and have a length that is a power of two.
///
///
pub fn strassen_square_matrix_multiply(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // Base case for a 2x2 matrix.

    // Divide the input matrices into n/2 submatrices.

    // Create ten matrices that are comprised of either the sum or difference of two of the submatrices.

    // Create seven matrix products.

    // Calculate the output quadrants from the product matrices.

    // Put the output quadrants into a single matrix.
    unimplemented!("This method is not implemented")
}