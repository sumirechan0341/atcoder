use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        s: Chars
    };
    let mut max = s[..].into_iter().collect::<Vec<_>>();
    let mut min = s[..].into_iter().collect::<Vec<_>>();
    for i in 0..s.len() {
        let spost = s[..i].into_iter().collect::<Vec<_>>();
        let spre = s[i..].into_iter().collect::<Vec<_>>();
        let snew = vec![spre, spost].concat().into_iter().collect::<Vec<_>>();
        if snew > max {
            max = snew.clone();
        }
        if snew < min {
            min = snew.clone();
        }
    }
    println!("{}", min.into_iter().collect::<String>());
    println!("{}", max.into_iter().collect::<String>());
}