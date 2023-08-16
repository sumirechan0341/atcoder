use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize
    };
    let ans = dfs("".to_string(), n);
    for a in ans {
        println!("{}", a);
    }
}

fn dfs(acc: String, n: usize) -> Vec<String> {
    if acc.len() == n {
        return vec![acc];
    } else {
        let mut local = vec![];
        for d in dfs(acc.clone()+&"a", n) {
            local.push(d);
        }
        for d in dfs(acc.clone()+&"b", n) {
            local.push(d);
        }
        for d in dfs(acc+&"c", n) {
            local.push(d);
        }
        return local;
    }
}