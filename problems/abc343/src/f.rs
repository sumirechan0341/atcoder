use ac_library::{Monoid, Segtree};
use proconio::{input, marker::Chars};
use std::collections::HashMap;

struct P {}
impl Monoid for P {
    type S = ((i64, i64), (i64, i64));
    fn identity() -> Self::S {
        return ((0, 0), (0, 0));
    }
    fn binary_operation(x: &Self::S, y: &Self::S) -> Self::S {
        if (x.0).0 > (y.0).0 {
            if (x.1).0 > (y.0).0 {
                (x.0, x.1)
            } else if x.1 .0 == y.0 .0 {
                (x.0, (x.1 .0, x.1 .1 + y.0 .1))
            } else {
                (x.0, y.0)
            }
        } else if x.0 .0 == y.0 .0 {
            if x.1 .0 > y.1 .0 {
                ((x.0 .0, x.0 .1 + y.0 .1), x.1)
            } else if x.1 .0 == y.1 .0 {
                ((x.0 .0, x.0 .1 + y.0 .1), (y.1 .0, x.1 .1 + y.1 .1))
            } else {
                ((x.0 .0, x.0 .1 + y.0 .1), y.1)
            }
        } else {
            if y.1 .0 > x.0 .0 {
                (y.0, y.1)
            } else if y.1 .0 == x.0 .0 {
                (y.0, (y.1 .0, x.0 .1 + y.1 .1))
            } else {
                (y.0, x.0)
            }
        }
    }
}

pub fn main() {
    input! {
        n: usize,
        q: usize,
        mut an: [i64; n],
        queryq: [(usize, usize, usize); q]
    };
    let mut seg = Segtree::<P>::new(n);
    for i in 0..n {
        seg.set(i, ((an[i], 1), (0, 0)));
    }
    for query in queryq {
        if query.0 == 1 {
            seg.set(query.1 - 1, ((query.2 as i64, 1), (0, 0)));
        } else {
            println!("{}", seg.prod(query.1 - 1..query.2).1 .1);
        }
    }
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
