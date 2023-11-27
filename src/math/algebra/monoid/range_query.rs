use crate::math::algebra::monoid::*;

#[allow(unused)]
struct RangeMinQuery {}

#[allow(unused)]
impl Monoid<usize> for RangeMinQuery {
    fn e() -> usize {
        std::usize::MAX / 2
    }
    fn op(lhs: usize, rhs: usize) -> usize {
        lhs.min(rhs)
    }
}

#[allow(unused)]
struct RangeMaxQuery {}

#[allow(unused)]
impl Monoid<i128> for RangeMaxQuery {
    fn e() -> i128 {
        -1
    }
    fn op(lhs: i128, rhs: i128) -> i128 {
        lhs.max(rhs)
    }
}