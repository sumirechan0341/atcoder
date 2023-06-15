use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        mut spn: [(String, i32); n]
    };
    let total = spn.iter().map(|x| x.1).sum::<i32>();
    spn.sort_by_key(|a| a.1);
    spn.reverse();
    if total/2 + 1 <= spn[0].1 {
        println!("{}", spn[0].0);
        return;
    }
    else {
        println!("{}", "atcoder");
    }
}