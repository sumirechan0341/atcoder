use std::collections::HashMap;

use proconio::{input, marker::{Chars, Usize1}};

pub fn main() {
    input! {
        n: usize,
        mut an: [i64; n],
        q: usize
    };
    let mut now = -1i64;
    let mut diff = HashMap::<usize, i64>::new();
    let mut ans = vec![];

    let mut seg = LazySegmentTree::new(n, |x, y| x, 0i64);
    for i in 0..n {
        seg.update(i, i+1, an[i]);
    }
    
    for _q in 0..q {
        input! {
            query_id: i32
        }
        if query_id == 1 {
            input! {
                x: i64
            }
            seg.update(0, n+1, x);
        } else if query_id == 2 {
            input! {
                idx: Usize1,
                x: i64
            }
            seg.update(idx, idx+1, x);
        } else {
            input! {
                idx: Usize1
            }
            println!("{:?}", seg.dat);
            ans.push(seg.query(idx, idx+1).to_string());
        }
    }
    println!("{}", ans.join("\n"));
}

struct LazySegmentTree<T, F> {
    n: usize,
    dat: Vec<T>,
    op: F,
    e: T,
    lazy: Vec<T>,
}

impl<T: Copy + std::fmt::Debug + Eq, F: Fn(T, T) -> T> LazySegmentTree<T, F> {
    fn new(n: usize, op: F, e: T) -> Self {
        let mut n_ = 1;
        while n_ < n {
            n_ *= 2;
        }
        LazySegmentTree {
            n: n_,
            dat: vec![e; 2*n_-1],
            lazy: vec![e; 2*n_-1],
            op,
            e
        }
    }
    fn eval(&mut self, k: usize, l: usize, r: usize) {
        if self.lazy[k] != self.e {
            self.dat[k] = (self.op)(self.lazy[k], self.dat[k]);
            if r - l > 1 {
                self.lazy[2*k+1] = (self.op)(self.lazy[k], self.lazy[2*k+1]);
                self.lazy[2*k+2] = (self.op)(self.lazy[k], self.lazy[2*k+2]);
            }
            self.lazy[k] = self.e;
        }
    }
    fn update(&mut self, a: usize, b: usize, x: T) {
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
            self.update_sub(a, b, x, 2*k+1, l, (l+r)/2);
            self.update_sub(a, b, x, 2*k+2, (l+r)/2, r);
            self.dat[k] = (self.op)(self.dat[2*k+1], self.dat[2*k+2]);
        }
    }
    fn query(&mut self, a: usize, b: usize) -> T {
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
            let vl = self.query_sub(a, b, 2*k+1, l, (l+r)/2);
            let vr = self.query_sub(a, b, 2*k+2, (l+r)/2, r);
            return (self.op)(vl, vr);
        }
    }
}