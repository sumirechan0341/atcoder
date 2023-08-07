use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        an: [i32; n]
    };
    let mut mul4 = 0;
    let mut even = 0;
    let mut odd = 0;
    for i in 0..n {
        if an[i]%4 == 0 {
            mul4 += 1;
        } else if an[i]%2 == 0 {
            even += 1;
        } else {
            odd += 1;
        }
    }
    if n == 2 {
        if mul4 >= 1 || even == 2 {
            println!("{}", "Yes");
        } else {
            println!("{}", "No");
        }
        return;
    }
    if n == 3 {
        if mul4 >= 1 || even == 3 {
            println!("{}", "Yes");
        } else {
            println!("{}", "No");
        }
        return;
    }
    if ((odd == mul4+1) && even == 0) || mul4 >= odd {
        println!("{}", "Yes");
    } else {
        println!("{}", "No");
    }
    return;
}