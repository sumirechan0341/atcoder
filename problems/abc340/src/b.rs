use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        q: usize
    };
    let mut v = vec![];
    let mut ans = vec![];
    for _ in 0..q {
        input! {
            id: usize,
            x: usize
        }
        if id == 1 {
            v.push(x);
        } else {
            ans.push(v[v.len() - x]);
        }
    }
    for a in ans {
        println!("{}", a);
    }
}
