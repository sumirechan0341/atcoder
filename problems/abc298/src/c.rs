use proconio::{input, marker::Chars};
use std::collections::{HashMap, HashSet};
pub fn main() {
    input! {
        n: usize,
        q: usize,
    };
    type Box = usize;
    type Card = usize;
    let mut boxes = HashMap::<Box, Vec<Card>>::new();
    let mut card = HashMap::<Card, HashSet<Box>>::new();
    let mut ans_str = vec![];
    for _ in 0..q {
        input! {
            qtype: i32,
        };
        match qtype {
            1 => {
                input! {
                    i: usize, // card
                    j: usize // box
                };
                match boxes.get_mut(&j) {
                    Some(x) => {
                        x.push(i);
                    }
                    None => {
                        boxes.insert(j, vec![i]);
                    }
                }
                match card.get_mut(&i) {
                    Some(x) => {
                        x.insert(j);
                    },
                    None => {
                        let mut news = HashSet::<Box>::new();
                        news.insert(j);
                        card.insert(i, news);
                    }
                }
            },
            2 => {
                input! {
                    i: usize
                };
                match boxes.get_mut(&i) {
                    Some(x) => {
                        x.sort();
                        ans_str.push(x.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
                    }
                    None => {
                        
                    }
                }
            },
            _ => {
                input! {
                    i: usize
                };
                match card.get_mut(&i) {
                    Some(x) => {
                        let mut ans = x.iter().collect::<Vec<_>>();
                        ans.sort();
                        ans_str.push(ans.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
                    }
                    None => {
                    }
                }

            }
        }
    }
    for a in ans_str {
        println!("{}", a);
    }
}