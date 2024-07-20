use proconio::input;

pub fn main() {
    input! {
        n: usize,
        an: [i64; n],
    }

    let m = 998244353;
    let mut dp = vec![vec![vec![0; n]; n]; n];
    for i in 0..n {
        for j in i + 1..n {
            for k in 2..n {
                if i + k > j {
                    continue;
                }
            }
        }
    }
}
