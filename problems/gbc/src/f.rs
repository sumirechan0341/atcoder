use itertools::Itertools;
use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        an: [char; n],
        q: usize,
        lrq: [(usize, usize); q]
    };
    let mut s = vec![vec![0; n+1]; 3];
    for i in 0..n {
        if an[i] == 'G' {
            s[0][i+1] = s[0][i]+1;
            s[1][i+1] = s[1][i];
            s[2][i+1] = s[2][i];
        } else if an[i] == 'C' {
            s[0][i+1] = s[0][i];
            s[1][i+1] = s[1][i]+1;
            s[2][i+1] = s[2][i];
        } else {
            s[0][i+1] = s[0][i];
            s[1][i+1] = s[1][i];
            s[2][i+1] = s[2][i]+1;
        }
    }
    for &(l, r) in &lrq {
        let mut ans = vec![];
        let g = s[0][r]-s[0][l-1];
        let c = s[1][r]-s[1][l-1];
        let p = s[2][r]-s[2][l-1];
        // println!("g {} c {} p {}", g, c, p);
        if judge(g+1, c, p, 0) {
            ans.push('G');
        }
        if judge(g, c+1, p, 1) {
            ans.push('C');
        }
        if judge(g, c, p+1, 2) {
            ans.push('P');
        }
        println!("{}", if ans.len() == 0 { "-1".to_string() } else { ans.iter().join(" ") });
    }
    
}

fn judge(g: i32, c: i32, p: i32, me: i32) -> bool {
    if me == 0 {
        if c == 0 && p == 0 {
            return true;
        }
        if c >= g && p > g {
            return true;
        }
        if c == 0 {
            return p > g;
        }
        if p == 0 {
            return c >= g;
        }
        
        return false;
    }
    if me == 1 {
        if g == 0 && p == 0 {
            return true;
        }
        if g > c && p >= c {
            return true;
        }
        if g == 0 {
            return p >= c;
        }
        if p == 0 {
            return g > c;
        }
        return false;
    }
    if me == 2 {
        if c == 0 && g == 0 {
            return true;
        }
        if g >= p && c > p {
            return true;
        }
        if g == 0 {
            return c > p;
        }
        if c == 0 {
            return g >= p;
        }
        return false;
    }
    return false;
}