use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m]
    };
    let mut connected = vec![];
    let mut j = 0;
    let mut current = vec![];
    for i in 0..n {
        current.push(i+1);
        if j < m && a[j] == i + 1 {
            j += 1;
        } else {
            connected.push(current);
            current = vec![];
        }
    }
    println!("{}", connected.iter().map(|c| c.iter().rev()).flatten().map(|c| c.to_string()).collect::<Vec<String>>().join(" "));
    
}
