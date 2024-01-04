#[allow(unused)]
struct PascalTriangleOnFiniteField {
    size: usize,
    v: Vec<Vec<FiniteField>>,
}
#[allow(unused)]
impl PascalTriangleOnFiniteField {
    pub fn new(size: usize) -> Self {
        let mut v = vec![vec![FiniteField::new(0); size + 1]; size + 1];
        v[1][1] = FiniteField::new(1);
        for i in 1..size + 1 {
            for j in 0..i + 1 {
                if j == 0 || j == i {
                    v[i][j] = FiniteField::new(1);
                } else {
                    v[i][j] = v[i - 1][j - 1] + v[i - 1][j];
                }
            }
        }
        PascalTriangleOnFiniteField {
            size: size,
            v: v,
        }
    }
}
use std::ops::*;

const MOD: usize = 10usize.pow(9) + 7;

#[derive(Clone, Copy)]
#[allow(unused)]
struct FiniteField {
    value: usize,
}

#[allow(unused)]
impl FiniteField {
    pub fn new(x: usize) -> Self {
        FiniteField {
            value: x % MOD,
        }
    }

    // self != 0 が必要
    pub fn inv(&self) -> FiniteField {
        let (x, mut y) = axby(MOD, self.value);
        y %= MOD as isize;
        if y < 0 {
            y += MOD as isize;
        }
        FiniteField::new(y as usize)
    }

    // a^n (mod m)
    // O(log n)
    pub fn pow(&self, mut n: usize) -> FiniteField {
        let mut res = FiniteField::new(1);
        let mut a = self.clone();
        while n > 0 {
            if n % 2 == 1 {
                res *= a;
            }
            a *= a;
            n /= 2;
        }
        res
    }   
}

impl Add for FiniteField {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        FiniteField::new(
            self.value + rhs.value,
        )
    }
}
impl AddAssign for FiniteField {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
        self.value %= MOD;
    }
}
impl Sub for FiniteField {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        FiniteField::new(
            MOD + self.value - rhs.value,
        )
    }
}
impl SubAssign for FiniteField {
    fn sub_assign(&mut self, rhs: Self) {
        self.value += MOD - rhs.value;
        self.value %= MOD;
    }
}
impl Mul for FiniteField {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        FiniteField::new(
            self.value * rhs.value
        )
    }
}
impl MulAssign for FiniteField {
    fn mul_assign(&mut self, rhs: Self) {
        self.value *= rhs.value;
        self.value %= MOD;
    }
}
impl Div for FiniteField {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inv()
    }
}
impl DivAssign for FiniteField {
    fn div_assign(&mut self, rhs: Self) {
        self.value *= rhs.inv().value;
        self.value %= MOD;
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