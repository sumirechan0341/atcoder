use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        m: usize,
        an: [i64; n],
        bm: [usize; m]
    };
    let mut boxes = LazySegmentTree::new(n, |x, y| x + y, 0);
    for i in 0..n {
        boxes.update(i, i + 1, an[i]);
    }
    // 22056
    // 43161
    // 04272
    for i in 0..m {
        let ball = boxes.query((bm[i]) % n, bm[i] + 1);
        boxes.update(bm[i], bm[i] + 1, -ball);
        let q = ball / n as i64;
        let r = ball % n as i64;
        boxes.update(0, n, q);
        let now_index = (bm[i] + 1) % n;
        if now_index + r as usize >= n {
            boxes.update(now_index, n, 1);
            boxes.update(0, (now_index + r as usize) % n, 1);
        } else {
            boxes.update(now_index, now_index + r as usize, 1);
        }
    }
    for i in 0..n {
        println!("{}", boxes.query(i, i + 1));
    }
}

/// 0-indexed
struct LazySegmentTree<T, F> {
    n: usize,
    dat: Vec<T>,
    op: F,
    e: T,
    lazy: Vec<T>,
    // mapping: fn(T, G) -> T,
    // composition: fn(G, G) -> G,
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
            // mapping,
            // composition,
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
