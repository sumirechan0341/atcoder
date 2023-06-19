use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        a: u64,
        b: u64,
        s: Chars
    };
    let mut cost = std::u64::MAX;
    for i in 0..n {
        let news = s[i..].into_iter().collect::<String>() + &s[..i].into_iter().collect::<String>();
        let news_vec = news.chars().collect::<Vec<char>>();
        let mut change_count = 0;
        for i in 0..n/2 {
            if news_vec[i] != news_vec[n-i-1] {
                change_count += 1;
            }
        }
        let new_cost = (i as u64) * a + b * change_count;
        if cost > new_cost {
            cost = new_cost;
        }
    }
    println!("{}", cost);
}