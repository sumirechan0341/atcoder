use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        m: usize,
        an: [Chars; n],
        bm: [Chars; m]
    };
    for i in 0..n-m+1 {
        for j in 0..n-m+1 {
            let mut ok = true;
            for k in 0..m {
                for l in 0..m {
                    if an[i+k][j+l] != bm[k][l] {
                        ok = false;
                        break;
                    }
                }
                if !ok {
                    break;
                }
            }
            if ok {
                println!("{}", "Yes");
                return;
            }
        }
    }
    println!("{}", "No");
}