#[allow(unused)]
// O(√n)
pub fn is_prime(n: usize) -> bool {
    n != 1 && (2..).take_while(|i| i * i <= n).all(|i| n % i != 0)
}

// O(√n)
#[allow(unused)]
fn dividors(n: usize) -> Vec<usize> {
    let mut res = vec![];
    for i in 1.. {
        if i * i > n {
            break;
        }
        if n % i == 0 {
            if i * i == n {
                res.push(i)
            } else {
                res.push(i);
                res.push(n / i);
            }
        }
    }
    res.sort();
    res
}

// O(√n)
#[allow(unused)]
fn prime_factorization(mut n: usize) -> Vec<(usize, usize)> {
    let mut res = vec![];
    for i in 2.. {
        if i * i > n {
            break;
        }
        if n % i == 0 {
            let mut cnt = 1;
            n = n / i;
            while n % i == 0 {
                cnt += 1;
                n = n / i;
            }
            res.push((i, cnt));
        }
    }
    if n != 1 {
        res.push((n, 1));
    }
    res
}


