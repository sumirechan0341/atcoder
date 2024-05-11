use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        k: usize,
        an: [usize; n]
    };
    let mut ans = 0;
    let mut local_sum = 0;
    for i in 0..n {
        if an[i] + local_sum > k {
            ans += 1;
            local_sum = an[i];
        } else {
            local_sum += an[i];
        }
    }
    if local_sum > 0 {
        ans += 1;
    }
    println!("{}", ans);
}
