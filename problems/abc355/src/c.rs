use std::collections::HashSet;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        t: usize,
        at: [usize; t]
    };
    let mut tate_bingo = vec![HashSet::new(); n];
    let mut yoko_bingo = vec![HashSet::new(); n];
    let mut naname_bingo = HashSet::new();
    let mut gyaku_naname_bingo = HashSet::new();

    for i in 0..n * n {
        let x = i / n;
        let y = i % n;
        yoko_bingo[x].insert(i + 1);
        tate_bingo[y].insert(i + 1);
        if x == y {
            naname_bingo.insert(i + 1);
        }
        if x + y == n - 1 {
            gyaku_naname_bingo.insert(i + 1);
        }
    }
    for i in 0..t {
        yoko_bingo[(at[i] - 1) / n].remove(&at[i]);
        tate_bingo[(at[i] - 1) % n].remove(&at[i]);
        naname_bingo.remove(&at[i]);
        gyaku_naname_bingo.remove(&at[i]);
        if yoko_bingo[(at[i] - 1) / n].is_empty()
            || tate_bingo[(at[i] - 1) % n].is_empty()
            || naname_bingo.is_empty()
            || gyaku_naname_bingo.is_empty()
        {
            println!("{}", i + 1);
            return;
        }
    }
    println!("{}", -1);
}
