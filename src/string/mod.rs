#[allow(unused)]
fn is_palindrome(s: Vec<u8>) -> bool {
    let mut t = s.clone();
    t.reverse();
    s == t
}

