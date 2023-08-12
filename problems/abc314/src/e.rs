use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        m: usize,
    };
    let mut cpsp = vec![];
    for i in 0..n {
        input!{
            c: usize,
            p: usize,
            sp: [usize; p]
        };
        cpsp.push((c, p, sp));
    }
    // dp[i] := iポイントためるときの総コストと確率
    // let mut dp = vec![(0, 0.0); m+1];
    // dp[0] = (0, 1.0);
    // for i in 1..m+1 {
    //     for (c, p, sp) in &cpsp {
    //         for k in sp {
    //             if *k > i {
    //                 continue;
    //             }
    //             if dp[i-*k].1 != 0.0 {
    //                 println!("{}", dp[i-*k].1*(1.0/(sp.len() as f64)));
    //                 dp[i] = (dp[i].0 + dp[i-*k].0+c, dp[i].1 + dp[i-*k].1*(1.0/(sp.len() as f64)));
    //             }
    //         }
    //     }
    // }
    // println!("{:?}", dp);
    
}