mod range_query;

#[allow(unused)]
pub trait Monoid<S> {
    fn e() -> S;
    fn op(lhs: S,rhs: S) -> S;
}