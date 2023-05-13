use proconio::input;

pub fn main() {
    input! {
        h: usize,
        w: usize,
        sh: [String; h],
    }
    let mut count = 0;
    for i in 0..h {
        for j in 0..w {
            if sh[i].chars().nth(j).unwrap() == '#' {
                count += 1;
            }            
        }
    }
    println!("{}", count);
}