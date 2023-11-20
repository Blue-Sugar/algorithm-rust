use crate::math::rational::Rational;

#[allow(unused)]
// x.a[0]a[1]...a[n-1]b[0]...b[k-1]b[0]..b[k-1]... -> rational
fn repeating_decimal_to_rational(x: usize, a: Vec<usize>, b: Vec<usize>) -> Rational {
    let mut res = Rational::new(x, 1);
    for (i, &a) in a.iter().enumerate() {
        res = res + Rational::new(a, 10usize.pow((i + 1) as u32));
    }
    let mut repeat = Rational::new(0, 1);
    for (i, &b) in b.iter().enumerate() {
        repeat = repeat + Rational::new(b, 10usize.pow((a.len() + i + 1) as u32));
    }
    res = res + repeat * Rational::new(10usize.pow(b.len() as u32), 10usize.pow(b.len() as u32) - 1);
    res
}

#[allow(unused)]
fn rational_to_repeating_decimal(r: Rational) -> (usize, Vec<usize>, Vec<usize>) {
    let mut denom = r.denom;
    let mut numer = r.numer;

    let mut two = 0;
    while numer % 2 == 0 {
        two += 1;
        numer /= 2;
    }
    let mut five = 0;
    while numer % 5 == 0 {
        five += 1;
        numer /= 5;
    }

    let ten = two.max(five);
    let mul = 2usize.pow(ten - two)
        * 5usize.pow(ten - five);
    denom *= mul;

    let mut used = vec![-1; numer];
    let (mut l, mut m) = (0, 0);
    for i in 1.. {
        let tmp = pow_mod(10, i, numer);
        if used[tmp] != -1 {
            (l, m) = (used[tmp] as usize, i);
            break;
        } else {
            used[tmp] = i as isize;
        }
    }
    let k = m - l;
    let d = (10usize.pow(k as u32) - 1) / numer;
    let mut n = denom * d / (10usize.pow(k as u32) - 1);
    let mut r = denom * d % (10usize.pow(k as u32) - 1);
    let mut b_s = vec![];
    for i in 0..k {
        let tmp = r % 10;
        b_s.push(tmp);
        r /= 10;
    }
    b_s.reverse();
    let mut a_s = vec![];
    for i in 0..ten as usize {
        let tmp = n % 10;
        a_s.push(tmp);
        n /= 10;
    }
    a_s.reverse();
    (n, a_s, b_s)
}

// a^n (mod m)
// O(log n)
pub fn pow_mod(a: usize, mut n: usize, m: usize) -> usize {
    let mut res = 1;
    let mut a = a.clone();
    while n > 0 {
        if n % 2 == 1 {
            res = res * a;
            res %= m;
        }
        a = a * a;
        a %= m;
        n /= 2;
    }
    res
} 
