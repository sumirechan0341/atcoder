use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        x: u128
    };
    let mut al = vec![];
    for _ in 0..n {
        input! {
            l: usize,
            local: [u128; l]
        };
        al.push(local);
    }
    let ans = al.into_iter().fold(vec![1], |acc, y| gen(acc, y));
    println!("{}", ans.into_iter().filter(|&y| y == x).into_iter().count());
}
fn gen(v1: Vec<u128>, v2: Vec<u128>) -> Vec<u128> {
    let mut res = vec![];
    for va in v1 {
        for vb in &v2 {
            res.push(va*vb);
        }
    }
    return res;
}