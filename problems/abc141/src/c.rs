use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        k: i64,
        q: usize,
        aq: [usize; q]
    };
    let mut total = 0;
    let mut ans = vec![k; n];
    for a in aq {
        ans[a-1] += 1;
        total += 1;
    }
    for i in 0..n {
        if ans[i] - total <= 0 {
            println!("{}", "No");
        } else {
            println!("{}", "Yes");
        }
    }
}