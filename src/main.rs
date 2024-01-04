use itertools::Itertools;

mod graph;
mod digraph;
mod set;
mod math;
mod string;
mod shared;

fn main() {
    println!("Hello, world!");
    let pt = math::combinatorics::pascal_triangle::PascalTriangle::new(10);

    for i in 1..11 {
        println!("{}", pt.v[i].iter().join(" "));
    }
}
