use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        qn: [usize; n],
        an: [usize; n],
        bn: [usize; n]
    };
    let mut a = 1000000;
    for i in 0..n {
        if an[i] == 0 {
            continue;
        }
        if a > qn[i] / an[i] {
            a = qn[i] / an[i];
        }
    }
    let mut ans = 0;
    for i in 0..=a {
        let mut b = 1000000;
        for j in 0..n {
            let remain = qn[j] - i * an[j];
            if bn[j] == 0 {
                continue;
            }
            if remain / bn[j] < b {
                b = remain / bn[j];
            }
        }
        if ans < i + b {
            ans = i + b;
        }
    }
    println!("{}", ans);
}
