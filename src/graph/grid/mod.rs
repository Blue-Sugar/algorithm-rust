mod valued_grid;

use itertools::Itertools;

#[allow(unused)]
struct Grid {
    h: usize,
    w: usize,
}

#[allow(unused)]
impl Grid {
    fn new(h: usize, w: usize) -> Self {
        Grid { h: h, w: w }
    }

    fn neignbor4(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let d = [(0, 1), (0, !0), (1, 0), (!0, 0)];
        let res = d.iter().map(
            |&(dx, dy)| (x.wrapping_add(dx), y.wrapping_add(dy))).filter(
            |&(x, y)| x < self.h && y < self.w
            ).collect_vec()
            ;
        res
    }

    fn neignbor8(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let d = [(0, 1), (0, !0), (1, 0), (!0, 0), (1, 1), (!0, !0), (!0, 1), (1, !0)];
        let res = d.iter().map(
            |&(dx, dy)| (x.wrapping_add(dx), y.wrapping_add(dy))).filter(
            |&(x, y)| x < self.h && y < self.w
            ).collect_vec()
            ;
        res
    }
}