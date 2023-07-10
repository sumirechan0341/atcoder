use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: Chars
    };
    let d = n.iter().map(|x| x.to_digit(10).unwrap() % 3).filter(|&x| x != 0);
    let d_sum = d.clone().sum::<u32>();
    let d_count = d.clone().count();
    if  d_sum % 3 == 0 {
        println!("{}", "0");
        return;
    }
    if d_sum % 3 == 1 {
        if d_count < 3 && d_count == n.len() {
            println!("{}", -1);
            return;
        }
        if d.collect::<Vec<_>>().contains(&1) {
            println!("{}", "1");
            return;
        } else {
            println!("{}", "2");
            return;
        }
    }
    if d_sum % 3 == 2 {
        if d_count < 3 && d_count == n.len() {
            println!("{}", "-1");
            return;
        }
        if d.collect::<Vec<_>>().contains(&2) {
            println!("{}", "1");
            return;
        } else {
            println!("{}", "2");
            return;
        }
    }
}