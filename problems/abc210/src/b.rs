use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        sn: Chars
    };
    for i in 0..n {
        if sn[i] == '1' {
            if i % 2 == 0 {
                println!("{}", "Takahashi");
                return;
            } else {
                println!("{}", "Aoki");
                return
            }
        }
    }
}