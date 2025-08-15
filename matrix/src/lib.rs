#[derive(Debug)]
pub struct Matrix<T>(pub Vec<Vec<T>>);
use lalgebra_scalar::Scalar;

impl<T: Scalar> Matrix<T> {
    pub fn new() -> Matrix<T> {
        Matrix(vec![Vec::new()])
    }

    pub fn zero(row: usize, col: usize) -> Matrix<T> {
        let mut matrix = Matrix(Vec::new());
        for _ in 0..row {
            let mut current_row = Vec::new();
            for _ in 0..col {
                current_row.push(T::zero());
            }
            matrix.0.push(current_row);
        }
        matrix
    }

    pub fn identity(n: usize) -> Matrix<T> {
        let mut matrix = Matrix::new();
        for y in 0..n {
            if y > 0 {
                matrix.0.push(Vec::new());
            }
            for x in 0..n {
                if y == x {
                    matrix.0[y].push(T::one());
                } else {
                    matrix.0[y].push(T::zero());
                }
            }
        }
        matrix
    }
}
