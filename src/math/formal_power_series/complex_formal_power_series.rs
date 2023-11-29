use crate::math::complex::*;


#[allow(unused)]
pub struct ComplexFormalPowerSeries {
    deg: usize,
    pub coef: Vec<Complex>,
}

#[allow(unused)]
impl ComplexFormalPowerSeries {
    pub fn new(coef: &Vec<Complex>) -> Self {
        let n = coef.len() - 1;
        ComplexFormalPowerSeries { deg: n, coef: coef.clone() }
    }

    // deg + 1 = 1000..0000 (2) となっていることを要請する
    pub fn fast_fourier_transform(&mut self) {
        let mut len = (self.deg + 1).trailing_zeros() as usize;
        let mut q = Vec::with_capacity(len / 2);
        for k in (0..len).rev() {
            let m = 1 << k;
            q.clear();
            q.extend((0..m).map(
                |i| Complex::omega(i, 2 * m)
            ));
            for coef in self.coef.chunks_exact_mut(2 * m) {
                let (x, y) = coef.split_at_mut(m);
                for ((x, y), q) in x.iter_mut().zip(y.iter_mut()).zip(&q) {
                    let a = *x;
                    let b = *y;
                    *x = a + b;
                    *y = (a - b) * *q;
                }
            }
        }

    }
    // deg + 1 = 1000...000 (2) を要請する
    pub fn inverse_fast_fourier_transform(&mut self) {
        let mut len = (self.deg + 1).trailing_zeros() as usize;
        let mut q = Vec::with_capacity(len / 2);
        for k in 0..len {
            let m = 1 << k;
            q.clear();
            q.extend((0..m).map(
                |i| Complex::omega(2 * m - i, 2 * m)
            ));
            for coef in self.coef.chunks_exact_mut(2 * m) {
                let (x, y) = coef.split_at_mut(m);
                for ((x, y), q) in x.iter_mut().zip(y.iter_mut()).zip(&q) {
                    let a = *x;
                    let b = *y * *q;
                    *x = a + b;
                    *y = a - b;
                }
            }
        }
    }
}