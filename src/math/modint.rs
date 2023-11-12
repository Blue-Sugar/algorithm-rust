use std::ops::{Add, Sub, Mul};

#[derive(Clone, Copy)]
#[allow(unused)]
struct ModUsize<const M: usize> {
    value: usize,
}

#[allow(unused)]
impl<const M: usize> ModUsize<M> {
    pub fn new(n: usize) -> Self {
        ModUsize { value: n % M }
    }
}

impl<const M: usize> Add for ModUsize<M> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        ModUsize { value: (self.value + rhs.value) % M }
    }
}
impl<const M: usize> Sub for ModUsize<M> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        ModUsize { value: (M + self.value - rhs.value) % M }
    }
}
impl<const M: usize> Mul for ModUsize<M> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        ModUsize { value: self.value * rhs.value % M }
    }
}