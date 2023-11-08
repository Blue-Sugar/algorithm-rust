pub trait Monoid<S> {
    fn op(&self, rhs: &Self) -> Self;
    fn e() -> Self;
}

pub struct SegmentTree<S, T> {
    data: Vec<T>,
    size: usize,
    phantom: std::marker::PhantomData<S>,
}

impl<S, T> SegmentTree<S, T>
where
    T: Monoid<S> + Clone,
{
    pub fn new(size: usize) -> Self {
        let size = size.next_power_of_two();
        Self {
            data: vec![T::e(); 2 * size],
            size: size,
            phantom: std::marker::PhantomData
        }
    }
    pub fn build(a: &[T]) -> Self {
        let size = a.len().next_power_of_two();
        let mut data = vec![T::e(); 2 * size];
        data[size..(size + a.len())].clone_from_slice(a);
        for i in (1..size).rev() {
            data[i] = data[2 * i].op(&data[2 * i + 1]);
        }
        Self { data, size, phantom: std::marker::PhantomData }
    }
}

impl<S, T> SegmentTree<S, T>
where
    T: Monoid<S>,
{
    pub fn find(&self, l: usize, r: usize) -> T {
        assert!(l <= r && r <= self.size);
        if l == r {
            return T::e();
        }
        let mut x = l + self.size;
        let mut y = r + self.size;
        let mut p = T::e();
        let mut q = T::e();
        let data = &self.data;
        while x < y {
            if x & 1 == 1 {
                p = p.op(&data[x]);
                x += 1;
            }
            if y & 1 == 1 {
                y -= 1;
                q = data[y].op(&q);
            }
            x >>= 1;
            y >>= 1;
        }
        p.op(&q)
    }

    // a_x <- v
    pub fn set_at(&mut self, x: usize, v: T) {
        assert!(x < self.size);
        let mut x = x + self.size;
        let data = &mut self.data;
        data[x] = v;
        x >>= 1;
        while x >= 1 {
            data[x] = data[2 * x].op(&data[2 * x + 1]);
            x >>= 1;
        }
    }

    // l <= j, f(a_j) = true of minimum j
    pub fn max_right<F>(&mut self, l: usize, f: F) -> usize
    where
        F: Fn(&T) -> bool,
    {
        assert!(f(&T::e()));
        if l == self.size {
            return self.size;
        }
        assert!(l <= self.size);
        let mut l = l + self.size;
        let mut r = 2 * self.size;
        let mut s = T::e();
        while l < r {
            if l & 1 == 1 {
                let v = s.op(&self.data[l]);
                if !f(&v) {
                    while l < self.size {
                        l *= 2;
                        let p = s.op(&self.data[l]);
                        if f(&p) {
                            s = p;
                            l += 1;
                        }
                    }
                    return l - self.size;
                } else {
                    s = v;
                }
                l += 1;
            }
            l >>= 1;
            r >>= 1;
        }
        self.size
    }
}