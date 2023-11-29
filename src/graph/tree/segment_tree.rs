use crate::math::algebra::monoid::Monoid;



#[allow(unused)]
struct SegmentTree<S> 
{
    n: usize,
    size: usize,
    data: Vec<S>,
}

#[allow(unused)]
impl<S> SegmentTree<S>
where S: Clone + Copy + Monoid 
{
    fn new(a: &Vec<S>) -> Self {
        let n = a.len();
        let size = n.next_power_of_two();
        let mut data = vec![S::e(); 2 * size];
        for (i, &a) in a.iter().enumerate() {
            data[size + i] = a;
        }
        for i in (1..size).rev() {
            data[i] = data[2 * i].op(&data[2 * i + 1]);
        }
        SegmentTree { n: n, size: size, data: data }
    }

    // a_i <- x
    fn set_at(&mut self, i: usize, x: S) {
        self.data[i + self.size] = x;
        let mut j = (i + self.size) / 2;
        while j > 0 {
            self.data[j] = 
                self.data[j * 2].op(&self.data[j * 2 + 1]); 
            j /= 2;
        }
    }

    // [l, r) \Pi a_i
    fn find(&self, l: usize, r: usize) -> S {
        let mut l = l + self.size;
        let mut r = r + self.size;

        let mut l_ans = S::e();
        let mut r_ans = S::e();

        while l < r {
            if l % 2 == 1 {
                l_ans = l_ans.op(&self.data[l]);
                l += 1;
            }
            if r % 2 == 1{
                r -= 1;
                r_ans = self.data[r].op(&r_ans);
            }
            l /= 2;
            r /= 2;
        }
        l_ans.op(&r_ans)
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
        let mut sum = S::e();
        while {
            l >>= l.trailing_zeros();
            let v = sum.op(&self.data[l]);
            if !f(&v) {
                while l < self.size {
                    l <<= 1;
                    let v = sum.op(&self.data[l]);
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
