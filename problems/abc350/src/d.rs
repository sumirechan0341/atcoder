use std::collections::HashSet;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        m: usize,
        abm: [(usize, usize); m]
    };
    let mut uf = UnionFind::new(n);
    for i in 0..m {
        uf.unite(abm[i].0 - 1, abm[i].1 - 1);
    }
    let mut cnt = vec![0; n];
    for i in 0..n {
        cnt[uf.root(i)] += 1;
    }
    let mut ans = 0;
    for i in 0..n {
        if cnt[i] > 0 {
            ans += cnt[i] * (cnt[i] - 1) / 2;
        }
    }
    println!("{}", ans - m);
}

pub struct UnionFind {
    par: Vec<i64>,
    siz: Vec<u64>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            par: vec![-1; n],
            siz: vec![1; n],
        }
    }

    // 根を求める
    pub fn root(&mut self, x: usize) -> usize {
        if self.par[x] == -1 {
            return x; // x が根の場合は x を返す
        } else {
            self.par[x] = self.root(self.par[x] as usize) as i64;
            return self.par[x] as usize;
        }
    }

    // x と y が同じグループに属するかどうか (根が一致するかどうか)
    pub fn issame(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    // x を含むグループと y を含むグループとを併合する
    pub fn unite(&mut self, x: usize, y: usize) -> bool {
        // x, y をそれぞれ根まで移動する
        let mut x = self.root(x);
        let mut y = self.root(y);

        // すでに同じグループのときは何もしない
        if x == y {
            return false;
        }

        // union by size (y 側のサイズが小さくなるようにする)
        if self.siz[x] < self.siz[y] {
            // swap(x, y);
            let tmp = y;
            y = x;
            x = tmp;
        }

        // y を x の子とする
        self.par[y] = x as i64;
        self.siz[x] += self.siz[y];
        return true;
    }

    // x を含むグループのサイズ
    pub fn size(&mut self, x: usize) -> u64 {
        let r = self.root(x);
        return self.siz[r as usize];
    }
}
