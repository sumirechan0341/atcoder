use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: Chars
    };
    if n.len() % 2 == 1 {
        println!("{}", 10_i32.pow((n.len()/2) as u32)-1);
        return;
    } 
    let pre = &n[..n.len()/2].to_vec().iter().collect::<String>().parse::<i32>().unwrap();
    let post = &n[n.len()/2..].to_vec().iter().collect::<String>().parse::<i32>().unwrap();
    if pre <= post {
        println!("{}", pre);
    } else {
        println!("{}", pre-1);
    }
}