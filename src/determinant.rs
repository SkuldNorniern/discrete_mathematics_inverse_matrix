use crate::Matrix;
use crate::MatrixError;

// Calculate determinant recursively using cofactor expansion
pub fn determinant(matrix: &Matrix) -> Result<f64, MatrixError> {
    let n = matrix.len();
    
    if n == 0 || matrix[0].len() != n {
        return Err(MatrixError::NotSquare);
    }
    
    if n == 1 {
        return Ok(matrix[0][0]);
    }
    
    if n == 2 {
        return Ok(matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0]);
    }
    
    let mut det = 0.0;
    
    // Expand along the first row
    for col in 0..n {
        let minor = get_minor(matrix, 0, col);
        let cofactor = if col % 2 == 0 { 1.0 } else { -1.0 };
        det += cofactor * matrix[0][col] * determinant(&minor)?;
    }
    
    Ok(det)
}


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

// Calculate cofactor matrix
fn cofactor_matrix(matrix: &Matrix) -> Result<Matrix, MatrixError> {
    let n = matrix.len();
    let mut cofactor = vec![vec![0.0; n]; n];
    
    for i in 0..n {
        for j in 0..n {
            let minor = get_minor(matrix, i, j);
            let sign = if (i + j) % 2 == 0 { 1.0 } else { -1.0 };
            cofactor[i][j] = sign * determinant(&minor)?;
        }
    }
    
    Ok(cofactor)
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

