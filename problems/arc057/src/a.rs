use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        mut a: i128,
        k: i128
    };
    let goal = 2000000000000;
    if k == 0 {
        println!("{}", goal-a);
        return;
    }
    else {
        let mut count = 0;
        while a < goal {
            let inc = a*k + 1;
            a += inc;
            count += 1;
        }
        println!("{}", count);
        return;
    }
}