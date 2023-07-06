use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        an: [i64; n]
    };
    let mut count: Vec<i64> = vec![0; 200];
    for i in 0..n {
        count[(an[i]%200) as usize] += 1;
    }
    let mut ans = 0;
    for i in 0..200 {
        ans += count[i]*(count[i]-1)/2;
    }
    println!("{}", ans);
}