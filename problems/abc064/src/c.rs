use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        an: [usize; n]
    };
    let mut ans = vec![0; 8];
    let mut variable = 0;
    for a in an {
        if a >= 3200 {
            variable += 1;
        } else {
            ans[a/400] += 1;
        }
    }
    println!("{} {}", ans.iter().filter(|&x| x!=&0).count().max(1), ans.iter().filter(|&x| x!=&0).count()+variable);
}