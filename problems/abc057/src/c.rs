use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: i64
    };
    let mut min = 10;
    for i in 1.. {
        if i*i > n {
            break;
        }
        if n%i == 0 {
            if min > count_digit(n/i) {
                min = count_digit(n/i);
            }
        }
    }
    println!("{}", min);
}

fn count_digit(i: i64) -> i64 {
    let mut j = i;
    let mut ans = 0;
    while j != 0 {
        j /= 10;
        ans += 1;
    }
    return ans;
}