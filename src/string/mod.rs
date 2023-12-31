#[allow(unused)]
fn is_palindrome(s: Vec<u8>) -> bool {
    let mut t = s.clone();
    t.reverse();
    s == t
}

#[allow(unused)]
fn run_length_encoding<T: Eq>(a: Vec<T>) -> Vec<(T, usize)> {
    let mut a = a.into_iter().map(|a| (a, 1)).collect::<Vec<_>>();
    a.dedup_by(|a, b| {
        a.0 == b.0 && {
            b.1 += a.1;
            true
        }
    });
    a
}