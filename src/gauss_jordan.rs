// Gauss-Jordan elimination for inverse matrix calculation
use crate::Matrix;
use crate::MatrixError;
use crate::print_matrix;

// Get the augmented matrix [A | I] for Gauss-Jordan elimination
fn get_augmented_matrix(matrix: &Matrix) -> Result<Matrix, MatrixError> {
    let n = matrix.len();
    if cfg!(debug_assertions) {
        println!("원래 행렬:");
        print_matrix(matrix);
    }

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
    if cfg!(debug_assertions) {
        println!("확장된 행렬:");
        print_matrix(&augmented);
    }
    Ok(augmented)
}

// Find the pivot row (row with largest absolute value in column)
fn find_pivot_row(augmented: &Matrix, col: usize, start_row: usize) -> usize {
    let n = augmented.len();
    if cfg!(debug_assertions) {
        println!("원래 행렬의 열:");
        for i in 0..n {
            println!("{:?}", augmented[i][col]);
        }
    }
    let mut max_row = start_row;

    for row in start_row + 1..n {
        if augmented[row][col].abs() > augmented[max_row][col].abs() {
            max_row = row;
        }
    }
    if cfg!(debug_assertions) {
        println!("최대 행: {}", max_row);
    }
    max_row
}

// Eliminate a column using the pivot row
fn eliminate_column(
    augmented: &mut Matrix,
    pivot_row: usize,
    col: usize,
) -> Result<(), MatrixError> {
    let n = augmented.len();
    if cfg!(debug_assertions) {
        println!("원래 행렬:");
        print_matrix(augmented);
    }

    // Check if matrix is singular (pivot is too small)
    if augmented[pivot_row][col].abs() < 1e-10 {
        return Err(MatrixError::SingularMatrix);
    }

    // Scale pivot row to make pivot = 1
    let pivot = augmented[pivot_row][col];
    for j in 0..2 * n {
        augmented[pivot_row][j] /= pivot;
    }

    // Eliminate column in all other rows
    for row in 0..n {
        if row != pivot_row {
            let factor = augmented[row][col];
            for j in 0..2 * n {
                augmented[row][j] -= factor * augmented[pivot_row][j];
            }
        }
    }
    if cfg!(debug_assertions) {
        println!("소거 후 행렬:");
        print_matrix(augmented);
    }
    Ok(())
}

// Perform forward elimination on the augmented matrix
fn forward_elimination(augmented: &mut Matrix) -> Result<(), MatrixError> {
    let n = augmented.len();

    if cfg!(debug_assertions) {
        println!("전체 소거 전 행렬:");
        print_matrix(augmented);
    }

    for col in 0..n {
        // Find pivot row
        let pivot_row = find_pivot_row(augmented, col, col);

        // Swap rows if needed
        if pivot_row != col {
            augmented.swap(col, pivot_row);
        }

        // Eliminate column
        eliminate_column(augmented, col, col)?;
    }
    if cfg!(debug_assertions) {
        println!("전체 소거 후 행렬:");
        print_matrix(augmented);
    }
    Ok(())
}

// Calculate inverse matrix using Gauss-Jordan elimination
pub fn inverse(matrix: &Matrix) -> Result<Matrix, MatrixError> {
    // Create augmented matrix [A | I]
    let mut augmented = get_augmented_matrix(matrix)?;

    // Perform forward elimination
    forward_elimination(&mut augmented)?;

    // Extract the inverse matrix from the right half
    let n = augmented.len();
    let mut result = vec![vec![0.0; n]; n];

    for i in 0..n {
        for j in 0..n {
            result[i][j] = augmented[i][n + j];
        }
    }

    Ok(result)
}
