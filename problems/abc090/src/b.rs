use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        a: i32,
        b: i32
    };
    let mut count = 0;
    for i in a..=b {
        if is_palindrome(i) {
            count += 1;
        }
    }
    println!("{}", count);
}
fn is_palindrome(n: i32) -> bool {
    let nstr = n.to_string().chars().collect::<Vec<_>>();
    for i in 0..nstr.len()/2 {
        if nstr[i] != nstr[nstr.len()-1-i] {
            return false;
        }
    }
    return true;
}