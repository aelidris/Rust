use std::ops::Mul;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Clone> Matrix<T> {
    pub fn number_of_cols(&self) -> usize {
        if self.0.is_empty() {
            0
        } else {
            self.0[0].len()
        }
    }

    pub fn number_of_rows(&self) -> usize {
        self.0.len()
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        self.0[n].clone()
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        self.0.iter().map(|row| row[n].clone()).collect()
    }
}

impl<T: Clone + std::ops::Mul<Output = T> + std::iter::Sum<T>> Mul for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn mul(self, other: Matrix<T>) -> Option<Matrix<T>> {
        if self.number_of_cols() != other.number_of_rows() {
            return None;
        }

        let mut result = Vec::new();
        for i in 0..self.number_of_rows() {
            let mut new_row = Vec::new();
            for j in 0..other.number_of_cols() {
                let sum: T = self.row(i)
                    .iter()
                    .zip(other.col(j).iter())
                    .map(|(a, b)| a.clone() * b.clone())
                    .sum();
                new_row.push(sum);
            }
            result.push(new_row);
        }
        Some(Matrix(result))
    }
}