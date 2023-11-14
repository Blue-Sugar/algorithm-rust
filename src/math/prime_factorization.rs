#[allow(unused)]
// O(√n)
pub fn is_prime(n: usize) -> bool {
    n != 1 && (2..).take_while(|i| i * i <= n).all(|i| n % i != 0)
}
#[test]
fn is_prime_test() {
    assert_eq!(is_prime(17), true);
    assert_eq!(is_prime(75367), true);
    assert_eq!(is_prime(25), false);
    assert_eq!(is_prime(57), false);
}

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
#[test]
fn divisors_test() {
    assert_eq!(dividors(18), vec![1, 2, 3, 6, 9, 18]);
    assert_eq!(dividors(25), vec![1, 5, 25]);
    assert_eq!(dividors(17), vec![1, 17]);
}

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
#[test]
fn prime_factorization_test() {
    assert_eq!(prime_factorization(18), vec![(2, 1), (3, 2)]);
    assert_eq!(prime_factorization(25), vec![(5, 2)]);
    assert_eq!(prime_factorization(17), vec![(17, 1)]);
    assert_eq!(prime_factorization(34), vec![(2, 1), (17, 1)])

}

