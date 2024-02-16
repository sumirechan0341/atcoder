use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        mut an: [usize; n]
    };
    let mut l = vec![1; n];
    let mut r = vec![1; n];
    for i in 1..n {
        l[i] = (l[i - 1] + 1).min(an[i])
    }
    for i in (0..n - 1).rev() {
        r[i] = (r[i + 1] + 1).min(an[i])
    }
    let mut ans = 1;
    for i in 0..n {
        if ans < l[i].min(r[i]) {
            ans = l[i].min(r[i]);
        }
    }
    println!("{}", ans);
}
