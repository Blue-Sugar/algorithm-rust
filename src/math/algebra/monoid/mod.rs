mod range_query;

#[allow(unused)]
pub trait Monoid: Clone + Copy {
    fn e() -> Self;
    fn op(&self,rhs: &Self) -> Self;
}