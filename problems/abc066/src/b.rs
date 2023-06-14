use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        s: Chars
    };
    for i in (2..s.len()).step_by(2) {
        let news = &s[..s.len()-i];
        if news[..news.len()/2] == news[news.len()/2..] {
            println!("{}", s.len() - i);
            return;
        }
    }
}
