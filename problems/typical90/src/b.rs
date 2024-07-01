use itertools::Itertools;
use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize
    };
    if n % 2 == 1 {
        println!("{}", "");
        return;
    }
    let ans = parentheses(n / 2, n / 2, "".to_string());
    println!("{}", ans.iter().join("\n"));
}

fn parentheses(left: usize, right: usize, acc: String) -> Vec<String> {
    let mut l = acc.clone();
    let mut res = vec![];
    if left == 0 && right == 0 {
        return vec![acc];
    }
    if left > 0 {
        l += "(";
        res.extend(parentheses(left - 1, right, l));
    }
    let mut r = acc.clone();
    if right > left {
        r += ")";
        res.extend(parentheses(left, right - 1, r));
    }
    res
}
