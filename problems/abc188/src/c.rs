use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        a2n: [usize; 2_i32.pow(n as u32)]
    };
    let mut max_index = 0;
    for i in 0..2_i32.pow(n as u32) as usize {
        if a2n[max_index] < a2n[i] {
            max_index = i;
        }
    }
    let mut ia2n = a2n.iter().enumerate().collect::<Vec<_>>();
    let (left, right) = ia2n.split_at_mut(2_i32.pow((n-1) as u32) as usize);
    if max_index >= 2_i32.pow((n-1) as u32) as usize {
        left.sort_by_key(|x| x.1);
        left.reverse();
        println!("{}", left[0].0+1);
    } else {
        right.sort_by_key(|x| x.1);
        right.reverse();
        println!("{}", right[0].0+1);
    };
}