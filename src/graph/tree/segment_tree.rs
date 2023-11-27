use crate::math::algebra::monoid::Monoid;
use std::marker::PhantomData;


#[allow(unused)]
struct SegmentTree<S, T> 
{
    n: usize,
    size: usize,
    data: Vec<S>,
    phantom: PhantomData<T>,
}

#[allow(unused)]
impl<S, T> SegmentTree<S, T>
where S: Clone + Copy, T: Monoid<S> 
{
    fn new(a: &Vec<S>) -> Self {
        let n = a.len();
        let size = n.next_power_of_two();
        let mut data = vec![T::e(); 2 * size];
        for (i, &a) in a.iter().enumerate() {
            data[size + i] = a;
        }
        for i in (1..size).rev() {
            data[i] = T::op(data[2 * i], data[2 * i + 1]);
        }
        SegmentTree { n: n, size: size, data: data, phantom: PhantomData }
    }

    // a_i <- x
    fn set_at(&mut self, i: usize, x: S) {
        self.data[i + self.size] = x;
        let mut j = (i + self.size) / 2;
        while j > 0 {
            self.data[j] = 
                T::op(self.data[j * 2], self.data[j * 2 + 1]); 
            j /= 2;
        }
    }

    // [l, r) \Pi a_i
    fn find(&self, l: usize, r: usize) -> S {
        let mut l = l + self.size;
        let mut r = r + self.size;

        let mut l_ans = T::e();
        let mut r_ans = T::e();

        while l < r {
            if l % 2 == 1 {
                l_ans = T::op(l_ans, self.data[l]);
                l += 1;
            }
            if r % 2 == 1{
                r -= 1;
                r_ans = T::op(self.data[r], r_ans);
            }
            l /= 2;
            r /= 2;
        }
        T::op(l_ans, r_ans)
    }

    fn all_prod(&self) -> S {
        self.data[1]
    }

    // a_p
    fn get(&self, p: usize) -> S {
        self.data[self.size + p]
    }

    // 区間和が条件式を満たす最長の区間 [l, res) の間で満たすとできる。
    // 条件式は単調でなければならない。
    // よくわかってない。
    fn max_right<P>(&self, l: usize, f: P) -> usize
    where
        P: Fn(&S) -> bool,
    {
        if l == self.n {
            return self.n;
        }
        let mut l = self.size + l;
        let mut sum = T::e();
        while {
            l >>= l.trailing_zeros();
            let v = T::op(sum, self.data[l]);
            if !f(&v) {
                while l < self.size {
                    l <<= 1;
                    let v = T::op(sum, self.data[l]);
                    if f(&v) {
                        sum = v;
                        l += 1;
                    }
                }
                return l - self.size;
            }
            sum = v;
            l += 1;
            l.count_ones() > 1
        } {}
        self.n
    }
}
