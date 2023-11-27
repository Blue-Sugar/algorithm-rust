use crate::math::modint::*;


use std::ops::Div;

impl<const M: usize> Div for ModUsize<M> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        self * rhs.inv().unwrap()
    }
}