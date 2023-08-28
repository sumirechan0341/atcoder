use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        xyzn: [(i64, i64, usize); n]
    };
    let costs = xyzn.iter().map(|x| ((x.0+x.1+1)/2-x.0).max(0)).collect::<Vec<_>>();
    let sum_z = xyzn.iter().map(|x| x.2).sum::<usize>();
    let mut dp = vec![vec![std::i64::MAX/2; sum_z+1]; n+1];
    dp[0][0] = 0;
    for i in 0..n {
        for j in 0..sum_z+1 {
            if j >= xyzn[i].2 {
                dp[i+1][j] = dp[i+1][j].min(dp[i][j-xyzn[i].2]+costs[i])
            } 
            dp[i+1][j] = dp[i+1][j].min(dp[i][j]);
        }
    }
    println!("{}", dp[n][(sum_z+1)/2..=sum_z].iter().min().unwrap());
}