use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        sn: [i64; n]
    };
    let mut ans: Vec<i64> = vec![];
    for i in 0..n {
        if i == 0 {
            ans.push(sn[0]);
        } else {
            ans.push(sn[i] - sn[i-1]);
        }
    }
    println!("{}", ans.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}