use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        w: usize,
        b: usize
    };
    let key = "wbwbwwbwbwbw".repeat(100);
    let cs = key.chars().collect::<Vec<_>>();
    for i in 0..=1000 {
        let mut wc = 0;
        let mut bc = 0;
        for j in 0..w + b {
            if cs[i + j] == 'w' {
                wc += 1;
            } else {
                bc += 1;
            }
        }
        if wc == w && bc == b {
            println!("{}", "Yes");
            return;
        }
    }
    println!("{}", "No");
}
