use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        an: [i32; n]
    };
    let mut v = vec![(1, an[0])];
    println!("1");
    for i in 1..n {
        if v.len() > 0 {
            let now = v[v.len()-1].1;
            let k = v[v.len()-1].0;
            if now == an[i] {
                v.push((k+1, an[i]));
            } else {
                v.push((1, an[i]));
            }
            if v[v.len()-1].0 == an[i] {
                while v.len() > 0 && v[v.len()-1].1 == an[i] {
                    v.pop();
                }
            }
        } else {
            v.push((1, an[i]));
        }
        println!("{}", v.len());
    }
}