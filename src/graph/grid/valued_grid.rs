use itertools::Itertools;

#[allow(unused)]
struct ValuedGrid<T> 
where T: Clone + Copy
{
    h: usize,
    w: usize,
    value: Vec<Vec<T>>,
}

#[allow(unused)]
impl<T> ValuedGrid<T> 
where T: Clone + Copy
{
    pub fn new(h: usize, w: usize, value: Vec<Vec<T>>) -> Self {
        ValuedGrid { h: h, w: w, value: value }
    }

    fn neignbor4(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let d = [(0, 1), (0, !0), (1, 0), (!0, 0)];
        d.iter().map(
            |&(dx, dy)| (x.wrapping_add(dx), y.wrapping_add(dy))).filter(
            |&(x, y)| x < self.h && y < self.w
            ).collect_vec()
    }

    fn neignbor8(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let d = [(0, 1), (0, !0), (1, 0), (!0, 0), (1, 1), (!0, !0), (!0, 1), (1, !0)];
        d.iter().map(
            |&(dx, dy)| (x.wrapping_add(dx), y.wrapping_add(dy))).filter(
            |&(x, y)| x < self.h && y < self.w
            ).collect_vec()      
    }

    fn neignbor4_value(&self, x: usize, y: usize) -> Vec<T> {
        self.neignbor4(x, y).iter().map(|&(x, y)| self.value[x][y])
        .collect_vec()
    }

    fn neignbor8_value(&self, x: usize, y: usize) -> Vec<T> {
        self.neignbor8(x, y).iter().map(|&(x, y)| self.value[x][y])
        .collect_vec()
    }
}