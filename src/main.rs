mod determinant;
mod gauss_jordan;

use std::io::{self, Write};
use std::env;

use discrete_mathematics_inverse_matrix::{Matrix, MatrixError, print_matrix, print_matrix_fraction, matrix_to_fraction};

fn main() {
    println!("=== Inverse Matrix Calculator ===\n");
    
    // Parse command-line arguments for fraction flag
    let args: Vec<String> = env::args().collect();
    let use_fractions = args.iter().any(|arg| arg == "--fraction" || arg == "-f");
    
    if use_fractions {
        println!("Mode: 분수 사용\n");
    } else {
        println!("Mode: 실수 사용n");
    }
    
    match run(use_fractions) {
        Ok(_) => {},
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
        },
        Err(MatrixError::SingularMatrix) => {
            println!("Error: Matrix is singular (determinant = 0). Inverse does not exist.");
            None
        },
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
        },
        Err(MatrixError::SingularMatrix) => {
            println!("Error: Matrix is singular. Inverse does not exist.");
            None
        },
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
    }
    
    Ok(())
}

fn read_matrix_size() -> Result<usize, String> {
    print!("Size of Matrix (n): ");
    io::stdout().flush().map_err(|e| e.to_string())?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(|e| e.to_string())?;
    
    let n: usize = input.trim().parse()
        .map_err(|_| "양의 정수를 입력하세요")?;
    
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
        io::stdin().read_line(&mut input).map_err(|e| e.to_string())?;
        
        let row: Result<Vec<f64>, _> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<f64>())
            .collect();
        
        let row = row.map_err(|_| format!("{}번째 행에 입력된 요소가 잘못됨. 공백으로 구분된 숫자를 입력하세요.", i + 1))?;
        
        if row.len() != n {
            return Err(format!("{}번째 행이 {}개의 요소를 가지고 있음, {}개 여야함", i + 1, row.len(), n));
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
