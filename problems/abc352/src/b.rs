use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        s: Chars,
        t: Chars
    };
    let mut cursor = 0;
    let mut ans = vec![];
    for i in 0..t.len() {
        if s[cursor] == t[i] {
            ans.push(i);
            cursor += 1;
        }
        if cursor == s.len() {
            break;
        }
    }
    println!(
        "{}",
        ans.iter()
            .map(|x| (x + 1).to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
