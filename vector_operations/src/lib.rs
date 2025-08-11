use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ThreeDVector<T> {
    pub i: T,
    pub j: T,
    pub k: T,
}

impl<T: Add<Output = T>> Add for ThreeDVector<T> {
    type Output = ThreeDVector<T>;

    fn add(self, rhs: ThreeDVector<T>) -> ThreeDVector<T> {
        ThreeDVector {
            i: self.i + rhs.i,
            j: self.j + rhs.j,
            k: self.k + rhs.k,
        }
    }
}

impl<T: Sub<Output = T>> Sub for ThreeDVector<T> {
    type Output = ThreeDVector<T>;

    fn sub(self, rhs: ThreeDVector<T>) -> ThreeDVector<T> {
        ThreeDVector {
            i: self.i - rhs.i,
            j: self.j - rhs.j,
            k: self.k - rhs.k,
        }
    }
}