use proconio::input;

pub fn main() {
    input! {
        s: String
    }
    println!("{}", s.chars().map(|c| if c == '0' { '1' } else { '0' }).collect::<String>());
}