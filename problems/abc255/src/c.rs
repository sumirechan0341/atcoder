use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        x: i128,
        a: i128,
        d: i128,
        n: i128
    };
    let start = a;
    let end = ak(a, d, n);
    if d >= 0 {
        if x >= end {
            println!("{}", x-end);
            return;
        }
        if x <= start {
            println!("{}", start-x);
            return;
        }
    }
    if d < 0 {
        if x <= end {
            println!("{}", end-x);
            return;
        }
        if x > start {
            println!("{}", x-start);
            return;
        }
    }
    // 2分探索自前で実装したやつ
    // d > 0 で昇順, d < 0 で降順バージョン
    let mut start_index = 1;
    let mut end_index = n;
    while end_index - start_index != 1 {
        let center_index = (start_index + end_index) / 2;
        if x > ak(a, d, center_index) {
            if d >= 0 {
                start_index = center_index;
            } else {
                end_index = center_index;
            }
            
        } else {
            if d >= 0 {
                end_index = center_index;
            } else {
                start_index = center_index;
            }
        }
    }

    println!("{}", (x-ak(a, d, start_index)).abs().min((x-ak(a, d, end_index)).abs()));

}

// 1-origin
fn ak(a: i128, d: i128, k: i128) -> i128 {
    return a+d*(k-1);
}