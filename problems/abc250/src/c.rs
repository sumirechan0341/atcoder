use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        q: usize,
        xq: [usize; q]
    };
    // 値 -> index
    let mut indices = vec![0; n+1];
    // index -> 値
    let mut values = vec![0; n+1];
    for i in 1..=n {
        indices[i] = i;
        values[i] = i;
    }
    for x in xq {
        let index = indices[x];
        if index == n {
            indices[values[index-1]] = index;
            indices[values[index]] = index-1;
            let tmp = values[index-1];
            values[index-1] = values[index];
            values[index] = tmp;
            
        } else {
            indices[values[index+1]] = index;
            indices[values[index]] = index+1;
            let tmp = values[index+1];
            values[index+1] = values[index];
            values[index] = tmp;
        }
        
        
    }
    println!("{}", values[1..].iter().map(|x| x.to_string()).collect::<VS>().join(" "));
}