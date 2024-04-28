use std::collections::{HashSet, VecDeque};

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        h: usize,
        w: usize,
        sh: [Chars; h]
    };
    let mut uf = UnionFind::new(h * w);
    let mut ans = vec![0; h * w];
    let mut used = vec![false; h * w];
    for i in 0..h {
        for j in 0..w {
            if !used[i * w + j] && sh[i][j] == '.' {
                let mut q = VecDeque::new();
                q.push_back((i, j));
                used[i * w + j] = true;
                if !check(&sh, i, j, h, w) {
                    continue;
                } else {
                    let dx = vec![0, 1, 0, -1];
                    let dy = vec![1, 0, -1, 0];
                    // 動けるマス目しかqueueに入って来ないと仮定
                    let mut local_used = HashSet::new();
                    while let Some((x, y)) = q.pop_front() {
                        for k in 0..4 {
                            let nx = (x as i64 + dx[k]).max(0).min((h - 1) as i64);
                            let ny = (y as i64 + dy[k]).max(0).min((w - 1) as i64);
                            let nx = nx as usize;
                            let ny = ny as usize;
                            if check(&sh, nx, ny, h, w) {
                                uf.unite(i * w + j, nx * w + ny);
                                if !used[nx * w + ny] {
                                    q.push_back((nx, ny));
                                    used[nx * w + ny] = true;
                                }
                            } else {
                                if local_used.contains(&(nx * w + ny)) {
                                    continue;
                                }
                                ans[i * w + j] += 1;
                                local_used.insert(nx * w + ny);
                            }
                        }
                    }
                }
            }
        }
    }
    println!(
        "{}",
        uf.siz.iter().zip(ans).map(|(a, b)| a + b).max().unwrap()
    );
}

fn check(v: &Vec<Vec<char>>, i: usize, j: usize, h: usize, w: usize) -> bool {
    let dx = vec![0, 1, 0, -1];
    let dy = vec![1, 0, -1, 0];
    for k in 0..4 {
        let nx = (i as i64 + dx[k]).max(0).min((h - 1) as i64);
        let ny = (j as i64 + dy[k]).max(0).min((w - 1) as i64);
        if v[nx as usize][ny as usize] != '.' {
            return false;
        }
    }
    return true;
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
