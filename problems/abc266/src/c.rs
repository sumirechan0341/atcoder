use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        abcdxy: [(i32, i32); 4]
    };
    let mut cross = vec![];
    for i in 0..4 {
        let v1 = (abcdxy[i].0 - abcdxy[(i+1)%4].0, abcdxy[i].1 - abcdxy[(i+1)%4].1);
        let v2 = (abcdxy[(i+1)%4].0 - abcdxy[(i+2)%4].0, abcdxy[(i+1)%4].1 - abcdxy[(i+2)%4].1);
        cross.push(v1.0*v2.1 - v1.1*v2.0 > 0);
    }
    println!("{}", if cross.iter().all(|x| *x) || cross.iter().all(|x| !x) { "Yes" } else { "No" });
}