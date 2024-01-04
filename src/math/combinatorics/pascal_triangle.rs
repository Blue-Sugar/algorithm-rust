#[allow(unused)]
pub struct PascalTriangle {
    size: usize,
    // v[n][r] = nCr
    pub v: Vec<Vec<usize>>,
}
#[allow(unused)]
impl PascalTriangle {
    pub fn new(size: usize) -> Self {
        let mut v = vec![vec![0; size + 1]; size + 1];
        v[1][1] = 1;
        for i in 1..size + 1 {
            for j in 0..i + 1 {
                if j == 0 || j == i {
                    v[i][j] = 1;
                } else {
                    v[i][j] = v[i - 1][j - 1] + v[i - 1][j];
                }
            }
        }
        PascalTriangle {
            size: size,
            v: v,
        }
    }
}