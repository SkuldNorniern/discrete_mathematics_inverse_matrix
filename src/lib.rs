pub type Matrix = Vec<Vec<f64>>;

#[derive(Debug)]
pub enum MatrixError {
    NotSquare,
    SingularMatrix,
}