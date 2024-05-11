use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        k: usize,
        pn: [usize; n]
    };
    let mut q = pn.iter().enumerate().collect::<Vec<(usize, &usize)>>();
    q.sort_by(|&a, &b| a.1.cmp(b.1));
    let mut seg1 = LazySegmentTree::new(n, |a, b| a.max(b), 0);
    let mut seg2 = LazySegmentTree::new(n, |a, b| a.min(b), 100000000);
    for i in 0..n {
        seg1.update(i, i + 1, q[i].0);
        seg2.update(i, i + 1, q[i].0);
    }
    let mut ans = 100000000;
    for i in 0..=n - k {
        if ans > seg1.query(i, i + k) - seg2.query(i, i + k) {
            ans = seg1.query(i, i + k) - seg2.query(i, i + k);
        }
    }
    println!("{}", ans);
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
