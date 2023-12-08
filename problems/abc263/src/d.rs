use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        l: i64,
        r: i64,
        an: [i64; n]
    };
    let mut sn = vec![0; n + 1];
    let mut cost = vec![0; n + 1];
    for i in 0..n {
        sn[i + 1] = sn[i] + an[i];
        cost[i + 1] = cost[i] + r - an[n - 1 - i];
    }
    let mut seg = LazySegmentTree::new(n + 1, |x, y| x.min(y), std::i64::MAX);
    for i in 0..=n {
        seg.update(i, i + 1, cost[i])
    }
    let mut min = sn[n];
    for i in 0..=n as i64 {
        let ls = l * i;
        let r = seg.query(0 as usize, n + 1 - i as usize);
        if min > ls + sn[n] - sn[i as usize] + r {
            min = ls + sn[n] - sn[i as usize] + r;
        }
    }
    println!("{}", min);
}

/// 0-indexed
struct LazySegmentTree<T, F> {
    n: usize,
    dat: Vec<T>,
    op: F,
    e: T,
    lazy: Vec<T>,
}

impl<T: Copy + std::fmt::Debug + Eq, F: Fn(T, T) -> T> LazySegmentTree<T, F> {
    pub fn new(n: usize, op: F, e: T) -> Self {
        let mut n_ = 1;
        while n_ < n {
            n_ *= 2;
        }
        LazySegmentTree {
            n: n_,
            dat: vec![e; 2 * n_ - 1],
            lazy: vec![e; 2 * n_ - 1],
            op,
            e,
        }
    }
    fn eval(&mut self, k: usize, l: usize, r: usize) {
        if self.lazy[k] != self.e {
            self.dat[k] = (self.op)(self.lazy[k], self.dat[k]);
            if r - l > 1 {
                self.lazy[2 * k + 1] = (self.op)(self.lazy[k], self.lazy[2 * k + 1]);
                self.lazy[2 * k + 2] = (self.op)(self.lazy[k], self.lazy[2 * k + 2]);
            }
            self.lazy[k] = self.e;
        }
    }
    pub fn update(&mut self, a: usize, b: usize, x: T) {
        self.update_sub(a, b, x, 0, 0, self.n);
    }

    fn update_sub(&mut self, a: usize, b: usize, x: T, k: usize, l: usize, r: usize) {
        self.eval(k, l, r);
        if r <= a || b <= l {
            return;
        }
        if a <= l && r <= b {
            self.lazy[k] = x;
            self.eval(k, l, r);
        } else {
            self.update_sub(a, b, x, 2 * k + 1, l, (l + r) / 2);
            self.update_sub(a, b, x, 2 * k + 2, (l + r) / 2, r);
            self.dat[k] = (self.op)(self.dat[2 * k + 1], self.dat[2 * k + 2]);
        }
    }
    pub fn query(&mut self, a: usize, b: usize) -> T {
        self.query_sub(a, b, 0, 0, self.n)
    }

    fn query_sub(&mut self, a: usize, b: usize, k: usize, l: usize, r: usize) -> T {
        self.eval(k, l, r);
        if r <= a || b <= l {
            return self.e;
        }
        if a <= l && r <= b {
            return self.dat[k];
        } else {
            let vl = self.query_sub(a, b, 2 * k + 1, l, (l + r) / 2);
            let vr = self.query_sub(a, b, 2 * k + 2, (l + r) / 2, r);
            return (self.op)(vl, vr);
        }
    }
}
