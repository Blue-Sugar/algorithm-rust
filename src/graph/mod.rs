mod tree;
mod grid;

#[allow(unused)]
pub struct Graph {
    // size of graph (|V|)
    n: usize,
    // adjoint list, i.e. v \in e[u] means there is a path between u and v.
    e: Vec<(usize, usize)>,
}

use itertools::Itertools;
#[allow(unused)]
impl Graph {
    pub fn new(n: usize, e: Vec<(usize, usize)>) -> Graph {
        Graph {
            n: n,
            e: e,
        }
    }
    pub fn adjoint_list(&self) -> Vec<Vec<usize>> {
        let mut res = vec![vec![]; self.n];
        for &(u, v) in &self.e {
            res[u].push(v);
            res[v].push(u);
        }
    res
    }
    // O(|V| + |E|)
    // ABC327-D accept
    fn is_biparate(&self) -> bool {
        let mut que = std::collections::VecDeque::new();
        let mut used = vec![-1; self.n];
        let adjoint_list = self.adjoint_list();
        let mut res = true;

        for i in 0..self.n {
            if used[i] != -1 {
                continue;
            }
            que.push_back(i);
            used[i] = 0;

            while let Some(u) = que.pop_front() {
                for &v in &adjoint_list[u] {
                    if used[u] == used[v] {
                        res = false;
                    }
                    if used[v] == -1 {
                        used[v] = 1 - used[u];
                        que.push_back(v)
                    }
                }
            }
        }
        res
    }

    // [a, b, c, ..] \in res means that e[a], e[b], ... is one of spanning tree
    // O(n * eCn) (e = |E|)
    // AC ABC328-E
    fn spanning_tree(&self) -> Vec<Vec<usize>> {
        let mut res = vec![];
        'lo: for ed in (0..self.e.len()).combinations(self.n - 1) {
            let mut adjoint_list = vec![vec![]; self.n];
            for &i in &ed {
                let u = self.e[i].0;
                let v = self.e[i].1;
                adjoint_list[u].push(v);
                adjoint_list[v].push(u);
            }
            let mut used = vec![false; self.n];
            let mut que = std::collections::VecDeque::new();
            que.push_back(0);
            used[0] = true;
            while let Some(u) = que.pop_front() {
                for &v in &adjoint_list[u] {
                    if used[v] {
                        continue
                    }
                    used[v] = true;
                    que.push_back(v);
                }
            }
            if used.iter().all(|x| *x) {
                res.push(ed);
            }
        }
        res
    }
}