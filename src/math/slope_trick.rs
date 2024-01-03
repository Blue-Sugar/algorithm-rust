// f: R -> R は以下の条件を満たす関数である。
// 1. 連続である。
// 2. 区分線形である。
// 3. 傾きは整数値である。
// 4. 下に凸である。

// f(x) = max(0, x - a) = (x - a)+
// f(x) = max(0, a - x) = (a - x)+
// このとき、|x - a| = (x - a)+ + (a - x)+

use std::cmp::Reverse;

const MAX: isize = std::isize::MAX / 2;
const MIN: isize = std::isize::MIN / 2;

// f(x) = min + \sum(l - x)+ + \sum(x - r)+
#[allow(unused)]
struct SlopeTrick {
    min: isize,
    l: std::collections::BinaryHeap<isize>,
    r: std::collections::BinaryHeap<Reverse<isize>>,
}

#[allow(unused)]
impl SlopeTrick {
    // f(x) = 0 として初期化する。
    pub fn new() -> Self {
        let mut l = std::collections::BinaryHeap::new();
        let mut r = std::collections::BinaryHeap::new();
        l.push(MIN);
        r.push(Reverse(MAX));
        SlopeTrick {
            min: 0,
            l: l, 
            r: r,
        }
    }

    // f(x) の最小値を取得する。
    pub fn min(&self) -> isize {
        self.min
    }

    // f(x) を f(x) + a (a: const) として更新する。
    pub fn add_const(&mut self, a: isize) {
        self.min += a;
    }

    // f(x) を f(x) + (x - a)+ (a: const) として更新する。
    pub fn add_plus(&mut self, a: isize) {
        self.min += (self.l.peek().unwrap() - a).max(0);
        self.l.push(a);
        let tmp = self.l.pop().unwrap();
        self.r.push(Reverse(tmp));
    }

    // f(x) を f(x) + (a - x)+ (a: const) として更新する。
    pub fn add_minus(&mut self, a: isize) {
        self.min += (a - self.r.peek().unwrap().0).max(0);
        self.r.push(Reverse(a));
        let tmp = self.r.pop().unwrap().0;
        self.l.push(tmp);
    }

    // f(x) を f(x) + |x - a| (a: const) として更新する。
    pub fn add_absolute(&mut self, a: isize) {
        self.add_plus(a);
        self.add_minus(a);
    }


}