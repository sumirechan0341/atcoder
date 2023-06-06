use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        xyn: [(i32, i32); n]
    };
    let mut distance = vec![];
    for i in 0..n {
        for j in 0..n {
            distance.push(norm2(xyn[i], xyn[j]));
        }
    }
    distance.sort();
    distance.reverse();
    println!("{}", (distance[0] as f32).sqrt());
}

fn norm2(p1: (i32, i32), p2: (i32, i32)) -> i32 {
    return (p1.0 - p2.0).pow(2) + (p1.1 - p2.1).pow(2);
}