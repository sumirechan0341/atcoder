use std::collections::VecDeque;

use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        an: [i32; n]
    };
    let mut ans = VecDeque::<i32>::new();
    // 文字列反転
    // シミュレーションしてるとTLEなら挿入の方向を制御して解決するかも
    for i in 0..n {
        if i%2 == 0 {
            ans.push_back(an[i]);
        } else {
            ans.push_front(an[i]);
        }
    }
    let mut ans_vec = ans.iter().map(|x| x.to_string()).collect::<VS>();
    if n%2 == 1 {
        ans_vec.reverse();
    }
    println!("{}", ans_vec.join(" "));
}