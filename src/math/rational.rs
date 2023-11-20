#[allow(unused)]
#[derive(Clone, Copy)]
struct Rational {
    // denom / numer
    // denominator
    denom: usize,
    // numerator
    numer: usize,
}

#[allow(unused)]
impl Rational {
    pub fn new(denom: usize, numer: usize) -> Self {
        let d = gcd(denom, numer);
        Rational { denom: denom / d, numer: numer / d }
    }
}

use std::ops::{ Add, Sub, Mul, Div };
impl Add for Rational {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let denom = self.denom * rhs.numer + self.numer * rhs.denom;
        let numer = self.numer * rhs.numer;
        Rational { denom: denom, numer: numer }
    }
}
impl Sub for Rational {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let denom = self.denom * rhs.numer - self.numer * rhs.denom;
        let numer = self.numer * rhs.numer;
        Rational { denom: denom, numer: numer }
    }
}
impl Mul for Rational {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let denom = self.denom * rhs.denom;
        let numer = self.numer * rhs.numer;
        Rational { denom: denom, numer: numer }
    }
}
impl Div for Rational {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        let denom = self.denom * rhs.numer;
        let numer = self.numer * rhs.denom;
        Rational { denom: denom, numer: numer }
    }
}

//O(log(max{a, b}))
#[allow(unused)]
fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {return a;}
    gcd(b, a % b)
}