use proconio::{input, marker::Chars};

pub fn main() {
    input !{
        n: usize,
        s: Chars,
        t: Chars
    };
    let mut a  = true;
    for i in 0..n {
        if (s[i] == t[i]) || ((s[i] == '1') && (t[i] == 'l')) || ((s[i] == 'l') && (t[i] == '1')) || ((s[i] == 'o') && (t[i] == '0')) || ((s[i] == '0') && (t[i] == 'o')) {
            
        } else {
            a = false
        }
    }
    println!("{}", if a { "Yes" } else { "No" });
}