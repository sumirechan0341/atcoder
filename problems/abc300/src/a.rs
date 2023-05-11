use proconio::input;

pub fn main() {
    input! {
        n: u32,
        a: u32,
        b: u32,
        cn: [u32; n],
    }
    
    for i in 0..n {
        if cn[i as usize] == a + b  {
            println!("{}", i + 1);
        }
    }
}