mod max_flow_graph;

#[allow(unused)]
struct WeightedDigraph {
    // size, |V|
    n: usize,
    // weighted edge, (u, v, w) <=> w(u, v) = w
    e: Vec<(usize, usize, usize)>,
}

#[allow(unused)]
impl WeightedDigraph {
    pub fn new(n: usize, e: Vec<(usize, usize, usize)>) -> WeightedDigraph {
        WeightedDigraph {
            n: n,
            e: e,
        }
    }
    // (v, w) \in res[u] <=> w(u, v) = w
    pub fn adjoint_list(&self) -> Vec<Vec<(usize, usize)>> {
        let mut res = vec![vec![]; self.n];
        for &(u, v, w) in &self.e {
            res[u].push((v, w));
        }
        res
    }
    // res[u][v] = x <=> the least time from u to v is x.
    pub fn floyd_warshall(&self) -> Vec<Vec<usize>> {
        let inf = std::usize::MAX / 2;
        let mut res = vec![vec![inf; self.n];  self.n];
        for &(u, v, w) in &self.e {
            res[u][v] = w;
        }
        for i in 0..self.n {
            res[i][i] = 0;
        }

        for k in 0..self.n {
            for u in 0..self.n {
                for v in 0..self.n {
                    res[u][v] = res[u][v].min(res[u][k] + res[k][v]);
                }
            }
        }
        res
    }
}