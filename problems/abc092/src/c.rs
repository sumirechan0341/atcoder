use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        mut an: [i32; n]
    };
    let mut total = 0;
    total += an[0].abs();
    for i in 1..n {
        total += (an[i]-an[i-1]).abs();
    }
    total += an[n-1].abs();
    an.push(0);
    an.insert(0, 0);
    for i in 1..=n {
        if (an[i-1] >= an[i] && an[i] >= an[i+1]) || (an[i-1] <= an[i] && an[i] <= an[i+1]) {
            println!("{}", total);
        } else {
            println!("{}", total-((an[i]-an[i-1]).abs()+(an[i]-an[i+1]).abs() - (an[i-1]-an[i+1]).abs()));
        }
    }
}