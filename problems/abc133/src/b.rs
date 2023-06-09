use std::collections::HashSet;
use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        d: usize,
        xdn: [[i32; d]; n]
    };
    let mut count = 0;
    let mut square_number: HashSet<i32> = HashSet::<i32>::new();
    for i in 0..1000 {
        square_number.insert((i as i32).pow(2));
    }
    for i in 0..n {
        for j in i+1..n {
            if square_number.contains(&norm2(&xdn[i], &xdn[j], d)) {
                count += 1;
            }
        }
    } 
    println!("{}", count);

}

fn norm2(x1: &Vec<i32>, x2: &Vec<i32>, n: usize) -> i32 {
    let mut sum = 0;
    for i in 0..n {
        sum += (x1[i] - x2[i]).pow(2);
    }
    return sum;
}