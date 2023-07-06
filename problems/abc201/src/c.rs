use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        s: Chars
    };
    let mut count = 0;
    let must_used = s.iter().enumerate().filter_map(|c| if c.1 == &'o' { Some(c.0) } else { None }).collect::<Vec<_>>();
    let must_not_used = s.iter().enumerate().filter_map(|c| if c.1 == &'x' { Some(c.0) } else { None }).collect::<Vec<_>>();
    for i in 0..10000 {
        let pass = format!("{:04}", i).chars().map(|c| c.to_digit(10).unwrap() as usize).collect::<Vec<_>>();
        if pass.iter().any(|c| must_not_used.contains(c)) {
            continue;
        }
        if must_used.iter().any(|c| !pass.contains(c)) {
            continue;
        }
        count += 1;
    }
    println!("{}", count);
}