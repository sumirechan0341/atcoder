use proconio::input;
pub fn main() {
    input !{
        n: usize,
        m: usize,
        a: [[usize; n]; m]
    };
    let mut friends = Vec::<(usize, usize)>::new();
    for j in 0..m {
        for k in 0..n-1 {
            let left = a[j][k].min(a[j][k+1]);
            let right = a[j][k].max(a[j][k+1]);
            if !friends.contains(&(left, right)) {
                friends.push((left, right));
            }
            
        }
    }
    println!("{}", n * (n - 1) / 2 -  friends.len());
}