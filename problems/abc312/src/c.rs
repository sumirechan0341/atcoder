use std::collections::BTreeMap;
use std::ops::Bound::Included;

use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        m: usize,
        mut an: [i32; n],
        mut bm: [i32; m]
    };
    // 値段が上がれば上がるほど売り手が増え、買い手が減る。
    // 買い手はbi円だと買うので、買わなくなるbi+1円が切れ目になる
    // 売り手が1増えるのと、買い手が1減るのは等価である
    // したがってM番目が答え
    // an.extend(bm.iter().map(|x| x+1));
    // an.sort();
    // println!("{}", an[m-1]);

    // 二部探索
    an.sort();
    bm.sort();
    let mut not_satisfy = 0;
    let mut satisfy = 1000000001;

    while satisfy - not_satisfy > 1 {
        let next = (not_satisfy + satisfy) / 2;
        let mut seller = 0;
        for i in 0..n {
            if next >= an[i] {
                seller += 1;
            }
        }
        let mut buyer = m;
        for i in 0..m {
            if next > bm[i] {
                buyer -= 1;
            }
        }
        if seller >= buyer {
            satisfy = next;
        } else {
            not_satisfy = next;
        }
    }
    println!("{}", satisfy);
}