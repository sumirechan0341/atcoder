use proconio::input;
use std::collections::VecDeque;
pub fn main() {
    input! {
        n: usize,
        k: usize,
        an: [String; n]
    }
    let mut queue: VecDeque<String> = VecDeque::from(an);
    for _i in 0..k {
        queue.pop_front();
        queue.push_back("0".to_string());
    }
    println!("{}", queue.iter().cloned().collect::<Vec<String>>().join(" "));   
}