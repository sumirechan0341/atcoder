use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        k: usize,
        an: [i32; n]
    };
    let mut ans = vec![0; n];
    for i in 0..k {
        let mut local = vec![];
        for j in (i..n).step_by(k) {
            local.push(an[j])
        }
        local.sort();
        for j in 0..local.len() {
            ans[i+j*k] = local[j];
        }
    }
    for i in 0..n-1 {
        if ans[i] > ans[i+1] {
            println!("{}", "No");
            return;
        }
    }
    println!("{}", "Yes");
}