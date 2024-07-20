use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
};

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: u64
    };
    if n == 1 {
        println!("{}", 0);
        return;
    }
    let mut length = 1;
    let mut count = 9;
    let mut num = n - 1;

    while num > count {
        num -= count;
        length += 1;
        count = 9 * 10u64.pow((length as u32 - 1) / 2);
    }

    let result = nth_palindrome(length, num);
    println!("{}", result);
}

fn nth_palindrome(length: usize, num: u64) -> String {
    let half_length = (length + 1) / 2;
    let base = 10u64.pow(half_length as u32 - 1) + num - 1;
    let base_str = base.to_string();
    let rev_str: String = if length % 2 == 0 {
        base_str.chars().rev().collect()
    } else {
        base_str.chars().rev().skip(1).collect()
    };
    let palindrome_str = base_str + &rev_str;
    palindrome_str
}
