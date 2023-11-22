use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        m: usize,
        s: Chars,
        t: Chars
    };
    let mut flag = false;
    let mut index = 0;
    let mut subset_t = vec![];
    let mut subset_t_rev = vec![];
    for i in 0..m {
        subset_t.push(&t[0..i+1]);
        subset_t_rev.push(&t[m-i-1..m]);
    }
    let mut index = 0;

    while true {
        let mut ok = false;
        if index+m > n {
            break;
        }
        for i in (0..m).rev() {
            if index + i > n {
                continue;
            }
            if subset_t.contains(&&s[index..=index+i]) {
                index += i+1;
                ok = true;
                break;
            } 
        }
        if !ok {
            break;
        } else if index == n {
            println!("{}", "Yes");
            return;
        }
    }
    let mut index2 = 0;
    while true {
        let mut ok = false;
        if index2+m > n {
            break;
        }
        for i in (0..m).rev() {
            if index2 + i > n {
                continue;
            }
            if subset_t_rev.contains(&&s[n-index2-i-1..n-index2]) {
                index2 += i+1;
                ok = true;
                break;
            } 
        }
        if !ok {
            break;
        } else if index2 == n {
            println!("{}", "Yes");
            return;
        }
    }
    if index + index2 < n {
        if n-index-index2 >= m {
            println!("{}", "No");
            return;
        } else {
            println!("{}", "Yes");
            return;
        }
    } else if index + index2 == n {
        if index%m + index2%m == m {
            println!("{}", "Yes");
            return;
        } else {
            println!("{}", "No");
            return;
        }
    } else {
        if s[index..index + (index+index2-n)] == t {
            println!("{}", "Yes");
            return;
        }
        println!("{}", "No");
        return;
    }
}