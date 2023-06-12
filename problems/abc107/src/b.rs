use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        h: usize,
        w: usize,
        ahw: [Chars; h]
    };
    // remove
    let mut temp = vec![];
    for ah in ahw {
        if ah.contains(&'#') {
            temp.push(ah);
        }
    }
    // 転置
    let mut temp2 = vec![];
    for i in 0..w {
        let mut local = vec![];
        for j in 0..temp.len() {
            local.push(temp[j][i]);
        }
        temp2.push(local);
    }
    // remove
    let mut ans = vec![];
    for a in temp2 {
        if a.contains(&'#') {
            ans.push(a);
        }
    }
    let ah = ans[0].len();
    let aw = ans.len();
    for i in 0..ah {
        for j in 0..aw {
            print!("{}", ans[j][i]);
        }
        println!("{}", "");
    }
}