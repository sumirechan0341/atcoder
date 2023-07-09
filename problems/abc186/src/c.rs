use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: i32
    };
    let mut count = 0;
    for i in 1..=n {
        if i.to_string().contains('7') || format!("{:0o}", i).contains('7') {
            continue;
        }
        count += 1;
    }
    println!("{}", count);
}