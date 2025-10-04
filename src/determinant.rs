// Get minor matrix by removing specified row and column
fn get_minor(matrix: &Matrix, row: usize, col: usize) -> Matrix {
    let n = matrix.len();
    let mut minor = Vec::new();
    
    for i in 0..n {
        if i == row {
            continue;
        }
        let mut new_row = Vec::new();
        for j in 0..n {
            if j == col {
                continue;
            }
            new_row.push(matrix[i][j]);
        }
        minor.push(new_row);
    }
    
    minor
}

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