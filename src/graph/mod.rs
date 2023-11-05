#[allow(unused)]
pub struct Graph {
    // size of graph (|V|)
    n: usize,
    // adjoint list, i.e. v \in e[u] means there is a path between u and v.
    e: Vec<(usize, usize)>,
}

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
}