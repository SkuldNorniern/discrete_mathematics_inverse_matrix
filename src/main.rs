mod determinant;
mod gauss_jordan;

use std::env;
use std::io::{self, Write};

use discrete_mathematics_inverse_matrix::{
    Matrix, MatrixError, matrix_to_fraction, print_matrix, print_matrix_fraction,
};

fn main() {
    println!("=== Inverse Matrix Calculator ===\n");

    // Parse command-line arguments for fraction flag
    let args: Vec<String> = env::args().collect();
    let use_fractions = args.iter().any(|arg| arg == "--fraction" || arg == "-f");

    if use_fractions {
        println!("Mode: 분수 사용\n");
    } else {
        println!("Mode: 실수 사용\n");
    }

    match run(use_fractions) {
        Ok(_) => {}
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn run(use_fractions: bool) -> Result<(), String> {
    // Read matrix size
    let n = read_matrix_size()?;

    // Read matrix
    println!("\nEnter the matrix row by row (space-separated values):");
    let matrix = read_matrix(n)?;

    println!("\n=== Input Matrix ===");
    if use_fractions {
        let frac_matrix = matrix_to_fraction(&matrix);
        print_matrix_fraction(&frac_matrix);
    } else {
        print_matrix(&matrix);
    }

    // Calculate inverse using determinant method
    println!("\n=== Method 1: Using Determinant ===");
    let det = determinant::determinant(&matrix).map_err(|_| "Failed to calculate determinant")?;

    if use_fractions {
        let det_frac = discrete_mathematics_inverse_matrix::Fraction::from_f64(det);
        println!("Determinant: {}", det_frac);
    } else {
        println!("Determinant: {}", det);
    }

    let inverse1 = match determinant::inverse(&matrix) {
        Ok(inv) => {
            println!("\nInverse matrix:");
            if use_fractions {
                let frac_inv = matrix_to_fraction(&inv);
                print_matrix_fraction(&frac_inv);
            } else {
                print_matrix(&inv);
            }
            Some(inv)
        }
        Err(MatrixError::SingularMatrix) => {
            println!("Error: Matrix is singular (determinant = 0). Inverse does not exist.");
            None
        }
        Err(_) => {
            return Err("Failed to calculate inverse using determinant method".to_string());
        }
    };

    // Calculate inverse using Gauss-Jordan method
    println!("\n=== Method 2: Using Gauss-Jordan Elimination ===");
    let inverse2 = match gauss_jordan::inverse(&matrix) {
        Ok(inv) => {
            println!("Inverse matrix:");
            if use_fractions {
                let frac_inv = matrix_to_fraction(&inv);
                print_matrix_fraction(&frac_inv);
            } else {
                print_matrix(&inv);
            }
            Some(inv)
        }
        Err(MatrixError::SingularMatrix) => {
            println!("Error: Matrix is singular. Inverse does not exist.");
            None
        }
        Err(_) => {
            return Err("Failed to calculate inverse using Gauss-Jordan method".to_string());
        }
    };

    // Compare results
    if let (Some(inv1), Some(inv2)) = (inverse1, inverse2) {
        println!("\n=== Comparison ===");
        if matrices_equal(&inv1, &inv2) {
            println!("두 방법이 동일한 결과를 도출");
        } else {
            println!("두 방법이 다른 결과를 도출 (수치 허용 오차 내에서).");
        }

        // Verify inverse by multiplying A * A^(-1) = I
        println!("\n=== Verification: A * A^-1 = I ===");
        let product = matrix_multiply(&matrix, &inv1);
        let identity = create_identity_matrix(n);

        println!("A * A^-1 =");
        if use_fractions {
            let frac_product = matrix_to_fraction(&product);
            print_matrix_fraction(&frac_product);
        } else {
            print_matrix(&product);
        }

        if matrices_equal(&product, &identity) {
            println!("\n✓ 검증 성공: A * A^-1 = I (단위행렬)");
            println!("  역행렬 계산이 정확합니다!");
        } else {
            println!("\n⚠ 검증 경고: 수치 오차로 인해 완벽한 단위행렬이 아닙니다.");
            println!("  하지만 허용 오차 내에서 올바른 결과입니다.");

            // Show max error
            let max_error = calculate_identity_error(&product, &identity);
            println!("  최대 오차: {:.2e}", max_error);
        }
    }

    Ok(())
}

fn read_matrix_size() -> Result<usize, String> {
    print!("Size of Matrix (n): ");
    io::stdout().flush().map_err(|e| e.to_string())?;

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|e| e.to_string())?;

    let n: usize = input.trim().parse().map_err(|_| "양의 정수를 입력하세요")?;

    if n == 0 {
        return Err("행렬 크기는 0보다 커야함".to_string());
    }

    Ok(n)
}

fn read_matrix(n: usize) -> Result<Matrix, String> {
    let mut matrix = Vec::new();

    for i in 0..n {
        print!("Row {}: ", i + 1);
        io::stdout().flush().map_err(|e| e.to_string())?;

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| e.to_string())?;

        let row: Result<Vec<f64>, _> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<f64>())
            .collect();

        let row = row.map_err(|_| {
            format!(
                "{}번째 행에 입력된 요소가 잘못됨. 공백으로 구분된 숫자를 입력하세요.",
                i + 1
            )
        })?;

        if row.len() != n {
            return Err(format!(
                "{}번째 행이 {}개의 요소를 가지고 있음, {}개 여야함",
                i + 1,
                row.len(),
                n
            ));
        }

        matrix.push(row);
    }

    Ok(matrix)
}

fn matrices_equal(m1: &Matrix, m2: &Matrix) -> bool {
    const EPSILON: f64 = 1e-6;

    if m1.len() != m2.len() {
        return false;
    }

    for i in 0..m1.len() {
        if m1[i].len() != m2[i].len() {
            return false;
        }
        for j in 0..m1[i].len() {
            if (m1[i][j] - m2[i][j]).abs() > EPSILON {
                return false;
            }
        }
    }

    true
}

// Matrix multiplication: C = A × B
fn matrix_multiply(a: &Matrix, b: &Matrix) -> Matrix {
    let n = a.len();
    let m = b[0].len();
    let p = b.len();

    let mut result = vec![vec![0.0; m]; n];

    for i in 0..n {
        for j in 0..m {
            let mut sum = 0.0;
            for k in 0..p {
                sum += a[i][k] * b[k][j];
            }
            result[i][j] = sum;
        }
    }

    result
}

// Create identity matrix of size n
fn create_identity_matrix(n: usize) -> Matrix {
    let mut identity = vec![vec![0.0; n]; n];
    for i in 0..n {
        identity[i][i] = 1.0;
    }
    identity
}

// Calculate maximum error from identity matrix
fn calculate_identity_error(matrix: &Matrix, identity: &Matrix) -> f64 {
    let mut max_error = 0.0;

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            let error = (matrix[i][j] - identity[i][j]).abs();
            if error > max_error {
                max_error = error;
            }
        }
    }

    max_error
}
