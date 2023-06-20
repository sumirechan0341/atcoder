use std::collections::HashSet;

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        q: usize,
        tabq: [(i32, usize, usize); q]
    };
    let mut friend = HashSet::<(usize, usize)>::new();
    for tab in tabq {
        match tab {
            (1, follow, followee) => {
                friend.insert((follow, followee));
            },
            (2, disfollow, disfollowee) => {
                friend.remove(&(disfollow, disfollowee));
            },
            (3, a, b) => {
                if friend.contains(&(a, b)) && friend.contains(&(b, a)) {
                    println!("{}", "Yes");
                } else {
                    println!("{}", "No");
                }
            }
            _ => {
                println!("{}", "unreachable");
            }
        }
    }
}