use proconio::input;

pub fn main() {
    input !{
        l1: i32,
        r1: i32,
        l2: i32,
        r2: i32,
    };
    if l2 > r1 || l1 > r2 {
        println!("{}", 0);
    } else {
        if (r2 < r1) ^ (l1 < l2)  {
            println!("{}", (r1 - l2).min(r2 - l1));
        } else {
            println!("{}", (r1 - l1).min(r2 - l2));
        }
    }
}