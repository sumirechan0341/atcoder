use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        s: Chars,
        t: Chars
    };
    let mut min = vec!['z'; 51];
    for i in 0..s.len()-t.len()+1 {
        let mut ok = true;
        let mut local = vec![];
        for j in 0..s.len() {
            if j < i || j -i >= t.len() {
                local.push(if s[j] == '?' {'a'} else { s[j] });
                continue;
            }
            if s[j] == t[j-i] || s[j] == '?' {
                local.push(t[j-i]);
            } else {
                ok = false;
                break;
            }
        }

        if ok {
            if min > local {
                min = local;
            }
        }
    }
    if min.len() == 51 {
        println!("{}", "UNRESTORABLE");
        return;
    }
    println!("{}", min.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(""));
}