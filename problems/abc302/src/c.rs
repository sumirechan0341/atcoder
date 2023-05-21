use proconio::{input, marker::Chars};

pub fn main() {
    input !{
        n: usize,
        m: usize,
        sn: [Chars; n]
    };
    let digit = 0;
    let mut perm: Vec<isize> = vec![-1; n];
    let mut perms: Vec<Vec<isize>> = vec![];
    let mut used_list = vec![];
    permutation(n, digit, &mut used_list, &mut perm, &mut perms);
    for index_vec in perms {
        let mut skip = false;
        for index_vec_index in 0..index_vec.len()-1 {
            let mut count = 0;
            for k in 0..m {
                if sn[index_vec[index_vec_index] as usize][k] != sn[index_vec[index_vec_index + 1] as usize][k] {
                    count += 1;
                }
            }
            if count != 1 {
                skip = true;
                break;
            }
        }
        if !skip {
            // 1文字違い
            println!("{}", "Yes");
            return;
        }
       
    }
    println!("{}", "No");
}
// 0-nの順列を列挙する関数
fn permutation(n: usize, digit: usize, used_list: &mut Vec<usize>, perm: &mut Vec<isize>, perms: &mut Vec<Vec<isize>>) {
    // digit桁目に格納する値を決める
    for i in 0..n {
        // 既に使われている値はdigit桁目に入れない
        if used_list.contains(&i) {
            continue
        }
        perm[digit] = i as isize;

        // n桁全て揃ったら組み合わせを格納して終了
        if digit == n-1 {
            perms.push(perm.clone());
        }

        // digit桁目(digit !=n-1)であれば、(digit+1)桁目に格納する値を決める
        else {
            used_list.insert(digit, i);
            permutation(n, digit + 1, used_list, perm, perms);
            used_list.remove(digit);
        }
    }
}