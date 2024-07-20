use proconio::{input, marker::Chars};
use std::collections::HashSet;

pub fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars
    };

    let mut ans = 0;
    let mut used = HashSet::new();

    let mut chars: Vec<char> = s.clone();
    chars.sort_unstable();

    loop {
        if !used.contains(&chars) {
            used.insert(chars.clone());

            if !contains_palindrome(&chars, k) {
                ans += 1;
            }
        }

        if !next_permutation(&mut chars) {
            break;
        }
    }

    println!("{}", ans);
}

fn contains_palindrome(s: &[char], k: usize) -> bool {
    s.windows(k).any(|window| is_palindrome(window))
}

fn is_palindrome(s: &[char]) -> bool {
    let len = s.len();
    for i in 0..len / 2 {
        if s[i] != s[len - 1 - i] {
            return false;
        }
    }
    true
}

fn next_permutation<T: Ord>(arr: &mut [T]) -> bool {
    let n = arr.len();
    if n < 2 {
        return false;
    }
    let mut i = n - 1;
    while i > 0 && arr[i - 1] >= arr[i] {
        i -= 1;
    }
    if i == 0 {
        return false;
    }
    let mut j = n - 1;
    while arr[j] <= arr[i - 1] {
        j -= 1;
    }
    arr.swap(i - 1, j);
    arr[i..].reverse();
    true
}
