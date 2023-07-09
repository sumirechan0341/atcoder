use std::collections::HashSet;

use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        sn: [Chars; n]
    };
    let mut used = HashSet::<Vec<char>>::new();

    for i in 0..n {
        if sn[i][0] == '!' {
            if used.contains(&sn[i][1..].to_vec()) {
                println!("{}", sn[i][1..].to_vec().iter().collect::<String>());
                return;
            }
        } else {
            let mut ex = sn[i].to_vec();
            ex.insert(0, '!');
            if used.contains(&ex) {
                println!("{}", sn[i].iter().collect::<String>());
                return;
            }
        }
        used.insert(sn[i].to_vec());
    }
    println!("{}", "satisfiable");
}