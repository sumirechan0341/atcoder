use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        hn: [i32; n]
    };
    println!("{}", dfs(hn));
}

fn dfs(target: Vec<i32>) -> i32 {
    if target.is_empty() {
        return 0;
    }
    if target.len() == 1 {
        return target[0];
    } else {
        for i in 0.. {
            let connected = target.split(|x| x == &i);
            if connected.clone().count() != 1 {
                return i + connected.fold(0, |acc, new_target| acc + dfs(new_target.iter().map(|x| x-i).collect::<Vec<_>>()));
            }
        }
        // unreachable
        return -500;
    }
}