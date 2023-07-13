use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        mut n: usize
    };
    let mut ans = vec![];
    while n > 0 {
        ans.push(((97 + ((n-1)%26) as u8) as char));
        n -= 1;
        n /= 26;
    }
    ans.reverse();
    println!("{}", ans.iter().collect::<String>());
}