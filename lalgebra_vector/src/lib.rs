use std::ops::Add;
use std::fmt::Debug;
use lalgebra_scalar::Scalar;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vector<T: Scalar>(pub Vec<T>);

impl<T: Scalar> Add for Vector<T> {
    type Output = Option<Vector<T>>;

    fn add(self, other: Vector<T>) -> Option<Vector<T>> {
        if self.0.len() != other.0.len() {
            return None;
        }
        let result = self.0.iter()
            .zip(other.0.iter())
            .map(|(a, b)| *a + *b)
            .collect();
        Some(Vector(result))
    }
}

impl<T: Scalar> Vector<T> {
    pub fn new() -> Self {
        Vector(Vec::new())
    }

    pub fn dot(&self, other: &Self) -> Option<T> {
        if self.0.len() != other.0.len() {
            return None;
        }
        let mut sum = T::zero();
        for (a, b) in self.0.iter().zip(other.0.iter()) {
            sum = sum + *a * *b;
        }
        Some(sum)
    }
}