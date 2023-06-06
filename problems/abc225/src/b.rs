use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: usize,
        abn: [(i32, i32); n-1]
    };
    let a1 = abn[0].0;
    let b1 = abn[0].1;

    let a2 = abn[1].0;
    let b2 = abn[1].1;
    
    if a1 != a2 && a1 != b2 && b1 != a2 && b1 != b2 {
        println!("{}", "No");
        return;
    }
    let c = if a1 == a2 || a1 == b2 { a1 } else { b1 };
    for ab in abn {
        if ab.0 != c && ab.1 != c {
            println!("{}", "No");
            return;
        }
    }
    println!("{}", "Yes");
}