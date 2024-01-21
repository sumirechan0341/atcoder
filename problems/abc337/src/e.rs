use itertools::Itertools;
use proconio::{input, input_interactive, marker::Chars};

pub fn main() {
    input_interactive! {
        n: usize
    };
    let mut m = 0;
    let mut i = 1;
    while i < n {
        m += 1;
        i = i << 1;
    }
    println!("{}", m);
    for i in 0..m {
        let mut local = vec![];
        for j in 1..=n {
            if j >> i & 1 == 1 {
                local.push((j).to_string());
            }
        }
        println!("{} {}", local.len(), local.iter().join(" "));
    }
    input_interactive! {
        mut s: Chars
    }
    s.reverse();
    let ans = usize::from_str_radix(&s.iter().collect::<String>(), 2).unwrap();
    if ans == 0 {
        println!("{}", n);
    } else {
        println!("{}", ans);
    }
}
