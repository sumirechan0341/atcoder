use proconio::input;

pub fn main() {
    input! {
        s: String
    }
    let mut count = 0;
    for c in s.chars() {
        if c == 'w' {
            count += 2;
        } else {
          count += 1;
        }
    }
    println!("{}", count);
}