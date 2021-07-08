fn square_matrix_multiply(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = a.len();
    let mut c: Vec<Vec<i32>> = Vec::new();
    for i in 1..n {
        for j in 1..n {
            c[i][j] = 0;
            for k in 1..n {
                c[i][j] = c[i][j] + a[i][k] * b[k][j];
            }
        }
    }
    c
}

// TODO Strassen Method, this is not explicitly in pseudo code