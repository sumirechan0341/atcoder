use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        q: usize,
        mut an: [i64; n],
        xq: [i64; q]
    };
    an.sort();
    let med = an[n/2];
    let mut min_cost = 0;
    for i in 0..n {
        min_cost += (an[i] - med).abs();
    }
    let mut sn = vec![0; n+1];
    for i in 0..n {
        sn[i+1] = sn[i] + an[i]-med;
    }
    // 最小値が0になるように平行移動する
    // or中央値が0
    let an2 = an.iter().map(|x| x*2).collect::<Vec<_>>();
    for x in xq {
        if let Err(y) = an2.binary_search(&(2*x-1)) {
            println!("y {}", y);
            println!("{}", (med-x).abs()*(n as i64-2*y as i64).abs()+min_cost);
        }
        // min_cost + (med-x)*(n-y) + 
        // println!("{}", ((x-med)*n as i64-d_med_sum).abs());
    }
    // N-5
    // -3 0 0 1 6
    // 
    
    // 1 => 24 損5
    // 2 => 19 得1 損4 x
    // 3 => 16 得1 損4
    // 4 => 13 得1 損4
    // 5 => 10 -3 0 0 1 6 x
    // 6 => 11 得2 損3 x
    // 7 => 14 得1 損4
    // 8 => 17
    // 9 => 20
    // 10 => 23
    // 11 => 26 x
    // 12 => 31
}