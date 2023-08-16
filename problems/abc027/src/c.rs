use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: i64
    };
    // 解説AC
    let mut bit_num = 0;
    let mut nn = n;
    while nn > 0 {
        bit_num += 1;
        nn /= 2;
    }
    let mut x = 1;
    let mut player = 0;
    if bit_num % 2 == 1 {
        while x <= n {
            if player%2 == 0 {
                x = x*2+1;
            } else {
                x = x*2;
            }
            player ^= 1;
        }
    } else {
        while x <= n {
            if player%2 == 1 {
                x = x*2+1;
            } else {
                x = x*2;
            }
            player ^= 1;
        }
    }
    println!("{}", if player == 0 { "Takahashi" } else { "Aoki" });
}