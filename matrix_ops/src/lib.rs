use std::ops::{Add, Sub};

#[derive(Debug, PartialEq)]
pub struct Matrix(pub Vec<Vec<i32>>);

impl Add for Matrix {
    type Output = Option<Matrix>;

    fn add(self, other: Matrix) -> Option<Matrix> {
        if self.0.len() != other.0.len() {
            return None;
        }

        let mut result = Vec::new();
        for (row_self, row_other) in self.0.iter().zip(other.0.iter()) {
            if row_self.len() != row_other.len() {
                return None;
            }
            let mut new_row = Vec::new();
            for (&a, &b) in row_self.iter().zip(row_other.iter()) {
                new_row.push(a + b);
            }
            result.push(new_row);
        }
        Some(Matrix(result))
    }
}

impl Sub for Matrix {
    type Output = Option<Matrix>;

    fn sub(self, other: Matrix) -> Option<Matrix> {
        if self.0.len() != other.0.len() {
            return None;
        }

        let mut result = Vec::new();
        for (row_self, row_other) in self.0.iter().zip(other.0.iter()) {
            if row_self.len() != row_other.len() {
                return None;
            }
            let mut new_row = Vec::new();
            for (&a, &b) in row_self.iter().zip(row_other.iter()) {
                new_row.push(a - b);
            }
            result.push(new_row);
        }
        Some(Matrix(result))
    }
}