use proconio::input;

pub fn main() {
    input !{
        a: [usize; 10]
    };
    let mut k = 0;
    for _i in 0..3 {
        k = a[k];
    }
    println!("{}", k);
}