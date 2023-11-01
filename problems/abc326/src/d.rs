use proconio::{input, marker::Chars};
use itertools::Itertools;
pub fn main() {
    input! {
        n: usize,
        r: Chars,
        c: Chars
    };
    for i in (0..n).permutations(n) {
        let mut masu = vec![vec!['.'; n]; n];
        let mut ok = true;
        for idx in 0..n {
            masu[idx][i[idx]] = 'A';
        }
        for j in (0..n).permutations(n) {
            let mut next = masu.clone();
            ok = true;
            for idx in 0..n {
                if next[idx][j[idx]] != '.' {
                    ok = false;
                    break;
                }
                next[idx][j[idx]] = 'B';
            }
            if !ok {
                continue;
            }
            for k in (0..n).permutations(n) {
                let mut next2 = next.clone();
                ok = true;
                for idx in 0..n {
                    if next2[idx][k[idx]] != '.' {
                        ok = false;
                        break;
                    }
                    next2[idx][k[idx]] = 'C';
                }
                if !ok {
                    continue;
                }
                for l in 0..n {
                    for m in 0..n {
                        if next2[l][m] == '.' {
                            continue;
                        }
                        if next2[l][m] == r[l] {
                            break;
                        }
                        ok = false;
                    }
                }
                for l in 0..n {
                    for m in 0..n {
                        if next2[m][l] == '.' {
                            continue;
                        }
                        if next2[m][l] == c[l] {
                            break;
                        }
                        ok = false;
                    }
                }
                if !ok {
                    continue;
                }
                println!("{}", "Yes");
                for j in 0..n {
                    for k in 0..n {
                        print!("{}", next2[j][k]);
                    }
                    println!("{}", "");
                }
                return;
            }
        }
    }
    
    println!("{}", "No");
}