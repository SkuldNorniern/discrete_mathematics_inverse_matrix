pub type Matrix = Vec<Vec<f64>>;


#[derive(Debug)]
pub enum MatrixError {
    NotSquare,
    SingularMatrix,
}

pub fn print_matrix(matrix: &Matrix) {
    for row in matrix {
        print!("[");
        for (j, val) in row.iter().enumerate() {
            if j > 0 {
                print!(" ");
            }
            print!("{:10.4}", val);
        }
        println!(" ]");
    }
}