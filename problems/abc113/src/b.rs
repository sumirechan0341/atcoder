use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: usize,
        t: i64,
        a: i64,
        mut hn: [i64; n]
    };
    let mut hn2 = hn.into_iter().enumerate().map(|x| (x.0, (1000*a - (1000*t - 6*x.1)).abs())).collect::<Vec<_>>();
    hn2.sort_by_key(|a| a.1); 
    println!("{}", hn2[0].0 + 1);
}