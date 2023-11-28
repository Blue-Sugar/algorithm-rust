mod binary_index_tree;
mod segment_tree;
mod union_find;

#[allow(unused)]
struct RootedTree {
    root: usize,
    // size, |V|
    n: usize,
    // edge set
    e: Vec<(usize, usize)>,
}

#[allow(unused)]
impl RootedTree {
    pub fn new(r: usize, n: usize, e: Vec<(usize, usize)>) -> Self {
        RootedTree { root: r, n: n, e: e }
    }

    pub fn adjoint_list(&self) -> Vec<Vec<usize>> {
        let mut res = vec![vec![]; self.n];
        for &(u, v) in &self.e {
            res[u].push(v);
            res[v].push(u);
        }
        res
    }

    // parent[i] = -1 のとき、それがrootであることを表す。
    pub fn parent(&self) -> Vec<isize> {
        let adjoint_list = self.adjoint_list();
        let mut res = vec![self.n as isize; self.n];
        let mut q = std::collections::VecDeque::new();
        q.push_back(self.root);
        res[self.root] = -1;

        while let Some(u) = q.pop_front() {
            for &v in &adjoint_list[u] {
                if res[v] != self.n as isize {
                    continue;
                }
                res[v] = u as isize;
                q.push_back(v);
            }
        }
        res
    }

    pub fn children(&self) -> Vec<Vec<usize>> {
        let mut res = vec![vec![]; self.n];
        for (i, &p) in self.parent().iter().enumerate() {
            if p == -1 {continue;}
            res[p as usize].push(i);
        }
        res
    }

    pub fn distance_from_root(&self) -> Vec<usize> {
        let adjoint_list = self.adjoint_list();
        let mut res = vec![std::usize::MAX; self.n];
        let mut q = std::collections::VecDeque::new();
        q.push_back(self.root);
        res[self.root] = 0;

        while let Some(u) = q.pop_front() {
            for &v in &adjoint_list[u] {
                if res[v] == std::usize::MAX {
                    res[v] = res[u] + 1;
                    q.push_back(v);
                }
            }
        }
        res
    }
}