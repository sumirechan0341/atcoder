use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        s: Chars
    };
    let mut index = (s.len()-1) as i32;
    let mut ans = 0;
    while index >= 0 {
        if s[index as usize] == '0' {
            if index == 0 {
                ans += 1;
                break;
            }
            if s[(index-1) as usize] == '0' {
                ans += 1;
                index -= 2;
                continue;
            }
        }
        index -= 1;
        ans += 1;
        
    }
    println!("{}", ans);
}