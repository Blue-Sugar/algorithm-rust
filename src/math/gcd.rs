//O(log(max{a, b}))
#[allow(unused)]
fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {return a;}
    gcd(b, a % b)
}

// O(log(max{a, b}))
#[allow(unused)]
fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

// O(log(max{a, b}))
#[allow(unused)]
fn is_coprime(a: usize, b: usize) -> bool {
    gcd(a, b) == 1
}

#[allow(unused)]
// O(log(max{a, b}))
// ax + by = d (d = gcd(a, b)), so return is (x, y)
fn axby(mut a: usize, mut b: usize) -> (isize, isize) {
    let mut history = vec![];
    while b > 0 {
        history.push((a, b));
        (a, b) = (b, a % b);
    }
    let (mut x, mut y) = (1, 0);
    for &(a, b) in history.iter().rev() {
        (x, y) = (y, x - a as isize / b as isize * y);
    }
    (x, y)
}