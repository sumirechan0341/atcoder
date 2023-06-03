use proconio::{input, marker::Chars};
use itertools::Itertools;
pub fn main() {
    input! {
        s: Chars
    };
    if s[0] == '1' {
        println!("{}", "No");
        return;
    }
    let rowc = (0..7).combinations(2);
    let rowp = vec![vec![7], vec![4], vec![2 ,8], vec![1, 5], vec![3, 9], vec![6], vec![10]];
    for c in rowc {
        // c[1] > c[0]のはず
        // 隣り合っている列は無視
        if c[1] - c[0] < 2 {
            continue;
        }
        let mut ng = true;
        // 各列にピンが残っているか確認
        for p in &rowp[c[1]] {
            if s[*p-1] == '1' {
                ng = false;
            }
        }
        if ng {
            continue;
        }
        ng = true;
        for p in &rowp[c[0]] {
            if s[*p-1] == '1' {
                ng = false;
            }
        }
        if ng {
            continue;
        }
         // 列の間のピンがすべて倒れているか確認
        let mut split_flag = true;
        for r in c[0]+1..c[1] {
            let pin_num_vec = &rowp[r];
            for pin_num in pin_num_vec {
                if s[*pin_num-1] == '1' {
                    split_flag = false;
                }
            }
        }
        if split_flag {
            println!("{}", "Yes");
            return;
        }
    }
    println!("{}", "No");
}