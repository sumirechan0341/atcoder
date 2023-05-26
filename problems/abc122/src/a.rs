use proconio::input;

pub fn main() {
    input! {
        b: char
    };
    println!("{}", if b == 'A' { 'T' } else if b == 'T' { 'A' } else if b == 'C' { 'G' } else { 'C' });
}
