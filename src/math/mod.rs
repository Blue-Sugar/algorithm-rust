mod modint;
mod algebra;
mod prime_factorization;
mod gcd;
mod eratosthenes;

#[allow(unused)]
// 0 <= x % m < m
fn safe_mod(x: isize, m: isize) -> isize {
    let mut res = x % m;
    if x < 0 {res += m}
    res
}

#[allow(unused)]
// x^n (mod m)
fn pow_mod(x: usize, mut n: usize, m: usize) -> usize {
    if m == 1 {return 0;}
    let mut res = 1;
    // x^t (t = 2^i)
    let mut y = x % m;

    while n > 0 {
        if n & 1 == 1 {
            res = res * y % m;
        }
        y = y * y % m;
        n >>= 1;
    }
    res % m
}

#[allow(unused)]
// inv_gcd(a, b) = (x, y) <=> (y / x) * a = gcd(a, b) (mod b)
fn inv_gcd(mut a: isize, b: isize) -> (isize, isize) {
    a = safe_mod(a, b);
    if a == 0 {return (b, 0);}
    let (mut s, mut t) = (b, a);
    let (mut m0, mut m1) = (0, 1);

    while t > 0 {
        let u = s / t;
        s -= t * u;
        m0 -= m1 * u;
        std::mem::swap(&mut s, &mut t);
        std::mem::swap(&mut m0, &mut m1);
    }

    if m0 < 0 {
        m0 += b as isize / s;
    }
    (s, m0)
}

#[allow(unused)]
// Miller-Rabin primary test by (2, 7, 61)
// minimun mistake result is 4759123141 = 48781*97561 > 2^32
fn is_prime_by_miller_rabin(n: usize) -> bool {
    if n <= 1 {return false;}
    if n == 2 || n == 7 || n == 61 {
        return true;
    }
    if n % 2 == 0 {return false;}
    let mut d = n - 1;
    while d % 2 == 0 {d /= 2}
    for &a in [2, 7, 61].iter() {
        let mut t = d;
        let mut y = pow_mod(a, t, n);
        while t != n - 1 && y != 1 && y != n - 1 {
            y = y * y % n;
            t <<= 1;
        }
        if y != n - 1 && t % 2 == 0 {
            return false;
        }
    }
    true
}

#[allow(unused)]
fn primitive_root(m: usize) -> usize {
    if m == 2 {return 1}
    if m == 167772161 {return 3}
    if m == 469762049 {return 3}
    if m == 754974721 {return 11}
    if m == 998244353 {return 3}

    let mut divs = vec![2];
    let mut x = (m - 1) / 2;
    while x % 2 == 0 {
        x /= 2;
    }
    let mut i = 3;
    while i * i <= x {
        if x % i == 0 {
            divs.push(i);
            while x % i == 0 {
                x /= i;
            }
        }
        i += 2;
    }
    if x > 1 {
        divs.push(x);
    }

    let mut g = 2;
    loop {
        if divs.iter().all(|div| pow_mod(g, (m - 1) / div, m) != 1) {
            return g
        }
        g += 1;
    }
}

#[allow(unused)]
fn inv_mod(x: usize, m: usize) -> Option<usize> {
    let z = inv_gcd(x as isize, m as isize);
    if z.0 != 1 {
        return None;
    }
    Some(z.1 as usize)
}

#[allow(unused)]
fn crt() {
    unimplemented!()
}

#[allow(unused)]
// floor(a * i + b) of sum i is moved in [0, n)
fn floor_sum_unsigned(
    n: usize, 
    m: usize, 
    mut a: usize, 
    mut b: usize) -> usize {
    let mut res = 0;
    if a >= m {
        res += n * (n - 1) * (a / m) / 2;
        a %= m;
    }
    if b >= m {
        res += n * (b / m);
        b %= m;
    }

    let y_max = a * n + b;
    if y_max < m {
        return res;
        }
    res += floor_sum_unsigned(y_max / m, a, m, y_max % m);
    res
}

#[allow(unused)]
fn floor_sum(
    n: usize, 
    m: usize, 
    mut a: isize, 
    mut b: isize) -> usize {
    let mut res = 0;
    if a < 0 {
        let a2 = a % m as isize;
        res -= n as isize * (n - 1) as isize * ((a2 - a) / m as isize) / 2;
        a = a2;
    }
    if b < 0 {
        let b2 = b % m as isize;
        res -= n as isize * ((b2 - b) / m as isize);
        b = b2;
    }

    res += floor_sum_unsigned(n, m, a as usize, b as usize) as isize;
    res as usize
}