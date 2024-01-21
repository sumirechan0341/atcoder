use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        mut s: Chars
    };
    s.dedup();
    if s.iter().collect::<String>() == "ABC".to_string()
        || s.iter().collect::<String>() == "AB".to_string()
        || s.iter().collect::<String>() == "BC".to_string()
        || s.iter().collect::<String>() == "AC".to_string()
        || s.iter().collect::<String>() == "A".to_string()
        || s.iter().collect::<String>() == "B".to_string()
        || s.iter().collect::<String>() == "C".to_string()
    {
        println!("{}", "Yes");
    } else {
        println!("{}", "No");
    }
}
