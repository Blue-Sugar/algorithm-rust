use std::ops::{Add, Sub, Mul};

#[derive(Clone, Copy)]
#[allow(unused)]
struct ModUsize<const M: usize> {
    value: usize,
}

#[allow(unused)]
impl<const M: usize> ModUsize<M> {
    pub fn new(n: usize) -> Self {
        ModUsize { value: n % M }
    }

    // a^n (mod m)
    // O(log n)
    pub fn pow(&self, mut n: usize) -> ModUsize<M> {
        let mut res = ModUsize::<M>::new(1);
        let mut a = self.clone();
        while n > 0 {
            if n % 2 == 1 {
                res = res * a;
            }
            a = a * a;
            n /= 2;
        }
        res
    }   

    pub fn inv(&self) -> Option<ModUsize<M>> {
        if gcd(self.value, M) != 1 {
            return None;
        }
        let (x, mut y) = axby(M, self.value);
        y %= M as isize;
        if y < 0 {
            y += M as isize;
        }
        Some(ModUsize::<M>::new(y as usize))
    }
}

impl<const M: usize> Add for ModUsize<M> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        ModUsize { value: (self.value + rhs.value) % M }
    }
}
impl<const M: usize> Sub for ModUsize<M> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        ModUsize { value: (M + self.value - rhs.value) % M }
    }
}
impl<const M: usize> Mul for ModUsize<M> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        ModUsize { value: self.value * rhs.value % M }
    }
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

//O(log(max{a, b}))
#[allow(unused)]
fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {return a;}
    gcd(b, a % b)
}