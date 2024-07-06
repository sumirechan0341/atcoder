use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        k: usize,
        x: usize,
        an: [usize; n]
    };
    println!(
        "{}",
        [
            [an.split_at(k).0, &vec![x]].concat(),
            an.split_at(k).1.to_vec()
        ]
        .concat()
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ")
    );
}
