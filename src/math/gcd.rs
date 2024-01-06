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

#[allow(unused)]
// a = a0 (mod m0), a = a1 (mod m1) <=> a = return + m0m1 k (k \in Z)
// return is the most smallest in natural.
// n0, m1: should be coprime.
fn chinese_remainder_theorem(
    a0: usize, 
    m0: usize,
    a1: usize,
    m1: usize,
) -> usize {
    let (x0, x1) = axby(m0, m1);
    let x = a0 as isize * m1 as isize * x1 + a1 as isize * m0 as isize * x0;
    let mut res = x % (m0 * m1) as isize;
    if res < 0 {
        res += (m0 * m1) as isize;
    }
    res as usize
}

#[allow(unused)]
fn is_square(x: usize) -> bool {
    let l = lower_bound(0, 10usize.pow(8),
        |k| k * k < x
    );
    l * l == x
}

#[allow(unused)]
// [l, r) がf = trueとなる数字である。
// f に単調性を課す。
fn lower_bound<P>(l: usize, ng: usize, f: P) -> usize  
where P: Fn(&usize) -> bool,
{ 
    if !f(&l) {return l;}
    let mut ok = l;
    let mut ng = ng;
    while ng - ok > 1 {
        let mid = (ok + ng) / 2;
        if f(&mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    ng
}