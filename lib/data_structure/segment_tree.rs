/// 0-indexed
struct SegmentTree<T, F> {
    n: usize,
    dat: Vec<T>,
    op: F,
    e: T,
}

impl<T: Copy + std::fmt::Debug, F: Fn(T, T) -> T> SegmentTree<T, F> {
    fn new(n: usize, op: F, e: T) -> Self {
        let mut n_ = 1;
        while n_ < n {
            n_ *= 2;
        }
        SegmentTree {
            n: n_,
            dat: vec![e; 2 * n_ - 1],
            op,
            e,
        }
    }
    /// k番目の値(0-indexed)をaに変更する
    fn update(&mut self, k: usize, a: T) {
        let mut k = k + self.n - 1;
        self.dat[k] = a;
        while k > 0 {
            k = (k - 1) / 2;
            self.dat[k] = (self.op)(self.dat[k * 2 + 1], self.dat[k * 2 + 2]);
        }
    }
    /// [a, b)の区間でクエリを実行する
    fn query(&self, a: usize, b: usize) -> T {
        self.query_sub(a, b, 0, 0, self.n)
    }

    /// queryの用の関数
    /// 陽に使うことはない
    fn query_sub(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> T {
        let mut ll = l + (self.n - 1);
        let mut rr = r + (self.n - 1);
        let mut y = self.e;
        while ll < rr {
            if ll & 1 == 0 {
                y = (self.op)(y, self.dat[ll]);
            }
            if rr & 1 == 0 {
                y = (self.op)(y, self.dat[rr - 1]);
            }
            ll = ll / 2;
            rr = (rr - 1) / 2;
        }
        return y;
    }
}

/// 0-indexed
struct LazySegmentTree<T, F> {
    n: usize,
    dat: Vec<T>,
    op: F,
    e: T,
    lazy: Vec<T>,
    mapping: fn(T, F) -> T,
    composition: fn(F, F) -> F,
}

impl<T: Copy + std::fmt::Debug + Eq, F: Fn(T, T) -> T> LazySegmentTree<T, F> {
    pub fn new(n: usize, op: F, e: T, mapping: fn(T, F) -> T, composition: fn(F, F) -> F) -> Self {
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
            mapping,
            composition,
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
