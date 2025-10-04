use crate::{Matrix,MatrixError,print_matrix};

// Calculate determinant recursively using cofactor expansion
pub fn determinant(matrix: &Matrix) -> Result<f64, MatrixError> {
    let n = matrix.len();

    if cfg!(debug_assertions) {
        println!("원래 행렬:");
        print_matrix(matrix);
    }
    if n == 0 || matrix[0].len() != n {
        return Err(MatrixError::NotSquare);
    }
    
    if n == 1 {
        if cfg!(debug_assertions) {
            println!("1x1 행렬의 행렬식: {}", matrix[0][0]);
        }
        return Ok(matrix[0][0]);
    }
    
    if n == 2 {
        let det = matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0];
        if cfg!(debug_assertions) {
            println!("2x2 행렬의 행렬식: ({} * {}) - ({} * {}) = {}", matrix[0][0], matrix[1][1], matrix[0][1], matrix[1][0], det);
        }
        return Ok(det);
    }
    
    let mut det = 0.0;
    
    // Expand along the first row
    for col in 0..n {
        let minor = get_minor(matrix, 0, col);
        let cofactor = if col % 2 == 0 { 1.0 } else { -1.0 };
        det += cofactor * matrix[0][col] * determinant(&minor)?;
    }
    if cfg!(debug_assertions) {
        println!("행렬식: {}", det);
    }
    Ok(det)
}


// Get minor matrix by removing specified row and column
fn get_minor(matrix: &Matrix, row: usize, col: usize) -> Matrix {
    let n = matrix.len();
    //if is debug mode, print the matrix
    if cfg!(debug_assertions) {
        println!("원래 행렬:");
        print_matrix(matrix);
    }
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
    if cfg!(debug_assertions) {
        println!("소행렬(row: {}, col: {}):", row, col);
        print_matrix(&minor);
    }
    minor
}

// Calculate cofactor matrix
fn cofactor_matrix(matrix: &Matrix) -> Result<Matrix, MatrixError> {
    let n = matrix.len();
    let mut cofactor = vec![vec![0.0; n]; n];
    if cfg!(debug_assertions) {
        println!("여인수를 구하기 전, 원래 행렬:");
        print_matrix(matrix);
    }
    for i in 0..n {
        for j in 0..n {
            let minor = get_minor(matrix, i, j);
            let sign = if (i + j) % 2 == 0 { 1.0 } else { -1.0 };
            cofactor[i][j] = sign * determinant(&minor)?;
        }
    }
    if cfg!(debug_assertions) {
        println!("여인수 행렬:");
        print_matrix(&cofactor);
    }
    Ok(cofactor)
}

// Transpose matrix
fn transpose(matrix: &Matrix) -> Matrix {
    let n = matrix.len();
    if cfg!(debug_assertions) {
        println!("전치 전 행렬:");
        print_matrix(matrix);
    }
    let mut result = vec![vec![0.0; n]; n];
    
    for i in 0..n {
        for j in 0..n {
            result[j][i] = matrix[i][j];
        }
    }
    if cfg!(debug_assertions) {
        println!("전치행렬:");
        print_matrix(&result);
    }
    result
}

// Calculate inverse matrix using determinant and adjugate matrix
pub fn inverse(matrix: &Matrix) -> Result<Matrix, MatrixError> {
    let det = determinant(matrix)?;

    
    // Check if matrix is singular
    if det.abs() < 1e-10 {
        return Err(MatrixError::SingularMatrix);
    }
    
    let n = matrix.len();
    
    // Special case for 1x1 matrix (return identity matrix)
    if n == 1 {
        return Ok(vec![vec![1.0 / matrix[0][0]]]);
    }
    
    // Calculate adjugate matrix (transpose of cofactor matrix)
    let cofactor = cofactor_matrix(matrix)?;
    let adjugate = transpose(&cofactor);
    
    // Divide by determinant to get inverse matrix
    let mut result = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..n {
            result[i][j] = adjugate[i][j] / det;
        }
    }

    Ok(result)
}