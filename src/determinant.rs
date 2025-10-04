// Transpose matrix
fn transpose(matrix: &Matrix) -> Matrix {
    let n = matrix.len();
    let mut result = vec![vec![0.0; n]; n];
    
    for i in 0..n {
        for j in 0..n {
            result[j][i] = matrix[i][j];
        }
    }
    
    result
}