#[allow(unused)]
struct Eratosthenes {
    n: usize,
    is_prime: Vec<bool>,
    min_factor: Vec<isize>,
}

#[allow(unused)]
impl Eratosthenes {
    // O(n log log n)
    pub fn new(n: usize) -> Self {
        let mut is_prime = vec![true; n + 1];
        let mut min_factor = vec![-1; n + 1];
        is_prime[0] = false;
        is_prime[1] = false;
        min_factor[0] = 0;
        min_factor[1] = 1;

        for i in 2..=n {
            if !is_prime[i] {
                continue;
            }
            min_factor[i] = i as isize;
            for j in 2.. {
                if i * j > n {
                    break;
                }
                is_prime[i * j] = false;
                if min_factor[i * j] == -1 {
                    min_factor[i * j] = i as isize;
                }
            }
        }
        Eratosthenes {
            n: n,
            is_prime: is_prime,
            min_factor: min_factor,
        }
    }
    fn prime_factorization(&self, mut m: usize) -> Vec<(usize, usize)> {
        let mut res = vec![];
        while m > 1 {
            let p = self.min_factor[m];
            let mut exp = 0;
            while self.min_factor[m] == p {
                m /= p as usize;
                exp += 1;
            }
            res.push((p as usize, exp));
        }
        res
    }
    fn dividors(&self, mut m: usize) -> Vec<usize> {
        let mut res = vec![1];
        let prime_factorization = self.prime_factorization(m);
        for &(p, exp) in &prime_factorization {
            for i in 0..res.len() {
                let mut v = 1;
                for j in 0..exp {
                    v *= p;
                    res.push(res[i] * v);
                }
            }
        }
        res.sort();
        res
    }
}

