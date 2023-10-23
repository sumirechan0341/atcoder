use itertools::Itertools;
use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        t: Chars,
        mut sn: [Chars; n]
    };
    let mut ans = vec![];
    for i in 0..n {
        if sn[i].len() == t.len() {
            if sn[i] == t {
                ans.push(i+1);
                continue;
            } else {
                let mut state = true;
                let mut ok = true;
                for j in 0..sn[i].len() {
                    if sn[i][j] != t[j] {
                        if state == false {
                            ok = false;
                            break;
                        }
                        state = false
                    }
                }
                if ok {
                    ans.push(i+1);
                }
            }
        } else if sn[i].len()-1 == t.len() {
            // クエリの文字の方が長い
            let mut offset = 0;
            let mut ok = true;
            for j in 0..sn[i].len()-1 {
                if sn[i][j] != t[j-offset] {
                    if offset == 1 {
                        ok = false;
                        break;
                    }
                    offset = 1;
                }
            }
            if offset == 0 || (ok && sn[i][sn[i].len()-1] == t[t.len()-1] ) {
                ans.push(i+1);
            }
        } else if sn[i].len() == t.len()-1 {
            // クエリの文字の方が短い
            let mut offset = 0;
            let mut ok = true;
            for j in 0..sn[i].len() {
                if sn[i][j-offset] != t[j] {
                    if offset == 1 {
                        ok = false;
                        break;
                    } 
                    offset = 1;
                }
            }
            if offset == 0 || (ok && sn[i][sn[i].len()-1] == t[t.len()-1] ) {
                ans.push(i+1);
            }
        }
    }
    println!("{}", ans.len());
    println!("{}", ans.iter().map(|x| x.to_string()).join(" "));
}