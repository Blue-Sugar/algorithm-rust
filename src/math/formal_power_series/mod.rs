mod complex_formal_power_series;
use crate::math::complex::*;

use std::ops::Mul;

use self::complex_formal_power_series::ComplexFormalPowerSeries;

#[allow(unused)]
struct FormalPowerSeries {
    deg: usize,
    coef: Vec<f64>,
}


#[allow(unused)]
impl FormalPowerSeries {
    fn new(coef: &Vec<f64>) -> Self {
        let n = coef.len() - 1;
        FormalPowerSeries { deg: n, coef: coef.clone() }
    }
}

impl Mul for FormalPowerSeries {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let n = self.deg + rhs.deg + 1;
        let k = n.next_power_of_two();
        let mut f = vec![Complex(0.0, 0.0); k];
        let mut g = vec![Complex(0.0, 0.0); k];
        for (f, lhs) in f.iter_mut().zip(self.coef) {
            f.0 = lhs;
        }
        for (g, rhs) in g.iter_mut().zip(rhs.coef) {
            g.0 = rhs;
        }
        let mut f_cfps = ComplexFormalPowerSeries::new(&f);
        let mut g_cpfs = ComplexFormalPowerSeries::new(&g);
        f_cfps.fast_fourier_transform();
        g_cpfs.fast_fourier_transform();
        for (f_cfps, g_cfps) in f_cfps.coef.iter_mut().zip(g_cpfs.coef) {
            *f_cfps = *f_cfps * g_cfps;
        }
        f_cfps.inverse_fast_fourier_transform();

        let mut ans = vec![0.0; n];
        for (ans, f_cfps) in ans.iter_mut().zip(f_cfps.coef) {
            *ans = f_cfps.0 / k as f64;
        }
        Self::new(&ans)
    }
}
