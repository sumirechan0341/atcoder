use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        an: [usize; n]
    };
    let mut v = vec![];
    for i in 0..n {
        v.push(an[i]);
        while v.len() >= 2 && v[v.len() - 2] == v[v.len() - 1] {
            let a = v.pop().unwrap();
            v.pop();
            v.push(a + 1);
        }
    }
    println!("{}", v.len());
}
