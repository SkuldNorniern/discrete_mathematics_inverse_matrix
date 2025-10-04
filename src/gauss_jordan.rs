// Gauss-Jordan elimination for inverse matrix calculation
use crate::Matrix;
use crate::MatrixError;

// Get the augmented matrix [A | I] for Gauss-Jordan elimination
fn get_augmented_matrix(matrix: &Matrix) -> Result<Matrix, MatrixError> {
    let n = matrix.len();

    if n == 0 || matrix[0].len() != n {
        return Err(MatrixError::NotSquare);
    }

    let mut augmented = vec![vec![0.0; 2 * n]; n];

    for i in 0..n {
        for j in 0..n {
            augmented[i][j] = matrix[i][j];
        }
        // Identity matrix on the right
        augmented[i][n + i] = 1.0;
    }

    Ok(augmented)
}

// Find the pivot row (row with largest absolute value in column)
fn find_pivot_row(augmented: &Matrix, col: usize, start_row: usize) -> usize {
    let n = augmented.len();
    let mut max_row = start_row;

    for row in start_row + 1..n {
        if augmented[row][col].abs() > augmented[max_row][col].abs() {
            max_row = row;
        }
    }

    max_row
}
