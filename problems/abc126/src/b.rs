use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        s: String
    };
    let aa = &s[0..2].parse::<i32>().unwrap();
    let bb = &s[2..4].parse::<i32>().unwrap();
    if aa > &12 || aa == &0 {
        // YYMMパターン
        if bb == &0 || bb > &12 {
            println!("{}", "NA");
            return;
        }
        println!("{}", "YYMM");
        return;
    }
    if bb > &12 || bb == &0 {
        // MMYYパターン
        if aa == &0 || aa > &12 {
            println!("{}", "NA");
            return;
        }
        println!("{}", "MMYY");
        return;
    }
    println!("{}", "AMBIGUOUS");
}