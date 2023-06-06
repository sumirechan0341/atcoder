use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        a: Chars,
        b: Chars
    };
    let d = a.len().min(b.len());
    for i in 0..d {
        let ad = a[a.len()-1-i].to_digit(10).unwrap();
        let bd = b[b.len()-1-i].to_digit(10).unwrap();
        if ad + bd > 9 {
            println!("{}", "Hard");
            return;
        }
    }
    println!("{}", "Easy");
}