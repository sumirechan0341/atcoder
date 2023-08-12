use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        s: Chars
    };
    let mut ans = vec![];
    for i in 0..1<<(s.len()-1) {
        let mut ii = i;
        let mut local: Vec<u64> = vec![];
        let mut acc = s[0].to_digit(10).unwrap() as u64;
        for j in 1..s.len() {
            if ii & 1 == 1 {
                local.push(acc);
                acc = s[j].to_digit(10).unwrap() as u64;
            } else {
                acc = acc*10 + s[j].to_digit(10).unwrap() as u64;
            }
            ii = ii >> 1;
        }
        local.push(acc);
        ans.push(local.iter().sum::<u64>());
    }
    println!("{}", ans.iter().sum::<u64>());
}