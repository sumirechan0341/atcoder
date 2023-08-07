use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: i64,
        m: i64
    };
    let p = 1000000007;
    if (n-m).abs() > 1 {
        println!("{}", 0);
        return;
    } else if (n-m).abs() == 1 {
        println!("{}", (mod_fact(n, p)*mod_fact(m, p))%p);
        return;
    }
    println!("{}", (2*(mod_fact(n, p)*mod_fact(m, p))%p));
}

fn mod_fact(n: i64, p: i64) -> i64 {
    let mut total = 1;
    for i in 2..=n {
        total *= i;
        total %= p;
    }
    return total;
}