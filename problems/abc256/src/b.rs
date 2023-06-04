use proconio::{input, marker::Chars};

pub fn main() {
    input !{
        n: usize,
        an: [i32; n]
    };
    let mut base: Vec<i32> = vec![];
    let mut score = 0;
    for a in an {
        let mut after = base.iter().map(|x| x + a).collect::<Vec<i32>>();
        after.push(a);
        let rem = after.clone().into_iter().filter(|x| x < &4).collect::<Vec<i32>>();
        score += after.len() - rem.len();
        base = rem;
    }
    println!("{}", score);
}