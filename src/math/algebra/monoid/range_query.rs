use crate::math::algebra::monoid::*;



#[allow(unused)]
impl Monoid for usize {
    fn e() -> usize {
        std::usize::MAX / 2
    }
    fn op(&self, rhs: &Self) -> usize {
        *self.min(rhs)
    }
}

#[allow(unused)]
impl Monoid for i128 {
    fn e() -> i128 {
        -1
    }
    fn op(&self, rhs: &Self) -> i128 {
        *self.max(rhs)
    }
}


#[allow(unused)]
fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {return a;}
    gcd(b, a % b)
}

#[allow(unused)]
impl Monoid for u128 {
    fn e() -> u128 {
        0
    }
    fn op(&self, rhs: &Self) -> u128 {
        gcd(*self, *rhs)
    }
}