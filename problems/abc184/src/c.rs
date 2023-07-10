use proconio::{input, marker::Chars, source};
type VS = Vec<String>;

pub fn main() {
    input!{
        r1: i32,
        c1: i32,
        r2: i32,
        c2: i32
    };
    let target = (r2-r1, c2-c1);
    if target.0 == 0 && target.1 == 0 {
        println!("{}", "0");
        return;
    }
    if target.0 == target.1 || target.0 == -target.1 {
        println!("{}", "1");
        return;
    }
    if target.0.abs() + target.1.abs() <= 3 {
        println!("{}", "1");
        return;
    }
    if target.0.abs() <= 5 && target.1.abs() <= 5 {
        println!("{}", "2");
        return;
    }
    if target.0 == 0 && target.1.abs() == 6 {
        println!("{}", "2");
        return;
    }
    if target.0.abs() == 6 && target.1 == 0 {
        println!("{}", "2");
        return;
    }
    if (target.0.abs()-target.1.abs()).abs() <= 3 {
        println!("{}", "2");
        return;
    }
    if target.0.abs() % 2 == target.1.abs() % 2 {
        println!("{}", "2");
        return;
    }
    println!("{}", "3");
}