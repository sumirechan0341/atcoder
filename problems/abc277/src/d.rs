use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        m: i64,
        mut an: [i64; n]
    };
    an.sort();
    let mut sn = vec![0; n+1];
    for i in 0..n {
        sn[i+1] = sn[i] + an[i];
    }
    let mut scc = vec![];
    let mut start = 0;
    let mut end = 0;
    for i in 0..n-1 {
        if an[i+1]-an[i] > 1 {
            scc.push((start, end+1));
            start = i+1;
            end = i+1;
        } else {
            end += 1;
        }
    }
    scc.push((start, end+1));

    let mut min = sn[n];
    for i in 0..scc.len()-1 {
        let now = sn[n]-(sn[scc[i].1]-sn[scc[i].0]);
        if now < min {
            min = now;
        }
    }
    if (an[n-1]+1)%m == an[0] && scc.len() > 1 {
        let now = sn[n]-(sn[scc[scc.len()-1].1]-sn[scc[scc.len()-1].0]+(sn[scc[0].1]-sn[scc[0].0]));
        if now < min {
            min = now;
        }
    } else {
        let now = sn[n]-(sn[scc[scc.len()-1].1]-sn[scc[scc.len()-1].0]);
        if now < min {
            min = now;
        }
    }
    println!("{}", min);
}