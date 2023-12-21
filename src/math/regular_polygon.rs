#[allow(unused)]
struct RegularPolygon {
    n: usize,
}

#[allow(unused)]
impl RegularPolygon {
    pub fn new(n: usize) -> Self {
        RegularPolygon { n: n }
    }

    // それが何個離れた対角線となっているかを調べる
    pub fn diagonal(&self, mut i: usize, mut j: usize) -> usize {
        if i > j {
            std::mem::swap(&mut i, &mut j);
        }

        (j - i).min(self.n + i - j)
    }
}