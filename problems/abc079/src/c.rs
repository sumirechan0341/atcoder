use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        abcd: Chars
    };
    for i in 0..8 {
        let mut ii = i;
        let mut total = abcd[0].to_digit(10).unwrap() as i32;
        let mut ans = abcd[0].to_string();
        for j in 0..3 {
            if ii & 1 == 1 {
                total += abcd[j+1].to_digit(10).unwrap() as i32;
                ans += &format!("+{}", abcd[j+1].to_string());
            } else {
                total -= abcd[j+1].to_digit(10).unwrap() as i32;
                ans += &format!("-{}", abcd[j+1].to_string()); 
            }
            ii = ii >> 1;
        }
        if total == 7 {
            println!("{}=7", ans);
            return;
        }
    }
}