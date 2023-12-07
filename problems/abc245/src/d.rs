use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        m: usize,
        mut an: [i32; n+1],
        mut cm: [i32; n+m+1]
    };

    let mut ans = vec![];
    for i in 0..=m {
        let d = cm[n + m - i] / an[n];
        ans.push(d.to_string());
        for j in 0..=n {
            cm[n + m - j - i] -= d * an[n - j];
        }
    }
    ans.reverse();
    println!("{}", ans.join(" "));
}
