use std::collections::VecDeque;
use std::iter::FromIterator;
use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        mut sa: Chars,
        mut sb: Chars,
        mut sc: Chars
    };
    let mut saq = VecDeque::from_iter(sa);
    let mut sbq = VecDeque::from_iter(sb);
    let mut scq = VecDeque::from_iter(sc);
    let mut next = saq.pop_front().unwrap();
    while true {
        match next {
            'a' => {
                match saq.pop_front() {
                    None => {
                        println!("{}", "A");
                        return;
                    },
                    Some(x) => {
                        next = x;
                    }
                }
            },
            'b' => {
                match sbq.pop_front() {
                    None => {
                        println!("{}", "B");
                        return;
                    },
                    Some(x) => {
                        next = x;
                    }
                }
            },
            'c' => {
                match scq.pop_front() {
                    None => {
                        println!("{}", "C");
                        return;
                    },
                    Some(x) => {
                        next = x;
                    }
                }
            },
            _ => {

            }
        }
    }
}