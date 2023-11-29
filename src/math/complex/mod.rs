use std::ops::*;

#[allow(unused)]
#[derive(Clone, Copy)]
pub struct Complex(pub f64, pub f64);

#[allow(unused)]
impl Complex {
    fn new(re: f64, im: f64) -> Self {
        Complex(re, im)
    }
    // exp(2πi * (denom / numer))
    pub fn omega(numer: usize, denom: usize) -> Self {
        let rad = 2.0 * std::f64::consts::PI * numer as f64 / denom as f64;
        Complex::new(rad.cos(), rad.sin())
    }
}

impl Add for Complex {
    type Output = Complex;
    fn add(self, rhs: Self) -> Self::Output {
        Complex(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Complex {
    type Output = Complex;
    fn sub(self, rhs: Self) -> Self::Output {
        Complex(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Mul for Complex {
    type Output = Complex;
    fn mul(self, rhs: Self) -> Self::Output {
        let Complex(a, b) = self;
        let Complex(c, d) = rhs;
        Complex(a * c - b * d, a * d + b * c)
    }
}