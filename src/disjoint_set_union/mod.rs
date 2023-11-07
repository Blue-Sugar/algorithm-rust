#[allow(unused)]
// AC atc001-B
pub struct DisjointSetUnion {
    // size of set (|S|)
    n: usize,
    // if p[i] < 0, then i is a root and the size is -p[i], else its parent is p[i]
    p: Vec<isize>,
}

#[allow(unused)]
impl DisjointSetUnion {
    pub fn new(n: usize) -> Self {
        DisjointSetUnion { n: n, p: vec![-1; n] }
    }

    pub fn root(&self, mut v: usize) -> usize {
        while self.p[v] >= 0 {
            v = self.p[v] as usize;
        }
        v
    }

    pub fn is_same(&self, u: usize, v: usize) -> bool {
        self.root(u) == self.root(v)
    }

    pub fn unite(&mut self, u: usize, v: usize) {
        let mut u = self.root(u);
        let mut v  = self.root(v);
        if u == v {return;}
        if self.p[u] > self.p[v] {
            std::mem::swap(&mut u, &mut v);
        }

        self.p[u] += self.p[v];
        self.p[v] = u as isize;
    }

    pub fn size(&self, v: usize) -> usize {
        -self.p[self.root(v)] as usize
    }
}