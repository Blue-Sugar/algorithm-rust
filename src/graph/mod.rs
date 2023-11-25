mod tree;
mod grid;

#[allow(unused)]
pub struct Graph {
    // order of graph (|V|)
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

    fn order(&self) -> usize {
        self.n
    }

    fn size(&self) -> usize {
        self.e.len()
    }

    pub fn adjoint_list(&self) -> Vec<Vec<usize>> {
        let mut res = vec![vec![]; self.n];
        for &(u, v) in &self.e {
            res[u].push(v);
            res[v].push(u);
        }
    res
    }

    // O(V^2 + E)
    pub fn incidence_matrix(&self) -> Vec<Vec<usize>> {
        let mut res = vec![vec![0; self.n]; self.n];
        for &(u, v) in &self.e {
            res[u][v] += 1;
            res[v][u] += 1;
        }
        res
    }

    // O(E)
    fn is_loop(&self) -> bool {
        self.e.iter().all(|&(u, v)| u == v)
    }

    // O(E logE)
    fn is_parallel(&self) -> bool {
        let mut al = self.adjoint_list();
        for i in 0..self.n {
            al[i].sort();
        }
        al.iter().all(|v| 
            v.windows(2).all(|v| v[0] == v[1])
        )
    }

    // O(E logE)
    fn is_simple(&self) -> bool {
        !self.is_loop() && !self.is_parallel()
    }

    // O(V^2 + E)
    fn is_complete(&self) -> bool {
        let im = self.incidence_matrix();
        im.iter().flatten().all(|cnt| *cnt > 0)
    }

    fn is_empty(&self) -> bool {
        self.e.len() == 0
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