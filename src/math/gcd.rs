#[allow(unused)]
fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {return a;}
    gcd(b, a % b)
}

#[allow(unused)]
fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

