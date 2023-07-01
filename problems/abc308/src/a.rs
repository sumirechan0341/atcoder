use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        mut s8: [i32; 8]
    };
    let s8c = s8.clone();
    s8.sort();
    println!("{}", if s8 == s8c && s8[0] >= 100 && s8[7] <= 675 && s8.iter().all(|x| x%25 == 0) { "Yes" } else { "No" });
}