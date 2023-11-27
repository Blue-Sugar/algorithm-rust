#[allow(unused)]
// a:vec の中で [l, r) が最小のfがtrueとなる区間である。
// f(l) = false のとき、Noneを返す。
fn lower_bound_in_vec<S, P>(a: &Vec<S>, l: usize, f: P) -> usize
where S: Clone + Copy, P: Fn(&S) -> bool,
{
    if !f(&a[l]) {return l;}
    let mut ok = l;
    let mut ng = a.len();
    while ng - ok > 1 {
        let mid = (ok + ng) / 2;
        if f(&a[mid]) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    ng
}


#[allow(unused)]
// [l, r) がf = trueとなる数字である。
// f に単調性を課す。
fn lower_bound<P>(l: i128, ng: i128, f: P) -> i128  
where P: Fn(&i128) -> bool,
{ 
    if !f(&l) {return l;}
    let mut ok = l;
    let mut ng = ng;
    while ng - ok > 1 {
        let mid = (ok + ng) / 2;
        if f(&mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    ng
}