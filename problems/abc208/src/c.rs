use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        k: u64,
        an: [usize; n]
    };
    // 逆順列の作り方
    let mut ian = an.iter().enumerate().collect::<Vec<_>>();
    ian.sort_by_key(|x| x.1);
    let mut inv = vec![0; n];
    for i in 0..n {
        inv[ian[i].0] = i;
    }
    let base = k / n as u64;
    let rem = k % n as u64;
    for i in 0..n {
        if inv[i] < rem as usize {
            println!("{}", base+1);
        } else {
            println!("{}", base);
        }
    }
}