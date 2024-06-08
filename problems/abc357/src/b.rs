use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        s: Chars
    };
    if s.iter().filter(|&c| c.is_ascii_uppercase()).count()
        > s.iter().filter(|&c| c.is_ascii_lowercase()).count()
    {
        println!("{}", s.iter().collect::<String>().to_uppercase());
    } else {
        println!("{}", s.iter().collect::<String>().to_lowercase());
    }
}
