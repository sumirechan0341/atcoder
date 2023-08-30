use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        mut n: usize
    };
    let card = vec!['1', '2', '3', '4', '5', '6'];
    let shift = n/5;
    let remain = n%5;
    let mut shifted = card.clone();
    for j in 0..6 {
        shifted[j] = card[(j+shift)%6];
    }
    for i in 0..remain {
        shifted.swap(i, i+1);
    }
    println!("{}", shifted.iter().collect::<String>());
}